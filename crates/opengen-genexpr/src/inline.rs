//! AST-level function inlining pass.
//!
//! Transforms a `Program` containing `FuncDecl` statements into one with all
//! calls to user-defined functions replaced by inlined code. After this pass:
//! - All `FuncDecl` statements are removed
//! - All `Return` statements inside function bodies are converted to assignments
//! - All calls to user functions in expressions are replaced by return-value identifiers
//! - `MultiAssign` from user-function calls are destructured into single assignments
//!
//! Renaming convention: `__inl<N>_<name>` where N is a per-call-site counter.
//! History declarations inside functions get fresh declarations per inline instance.
//!
//! # Early-return support decision (M2)
//! In M2, `return` is only legal as the sole or final top-level statement of a
//! function body. No-return functions are also accepted. Any return nested inside
//! If/While/DoWhile/For/Block, multiple returns, or a return followed by trailing
//! statements produces a clear `LowerError` containing "early return".
//!
//! Full early-return support (where return acts as an actual control-flow barrier)
//! is planned for M3 and will require rewriting the inliner to generate proper
//! branch-and-merge control flow instead of sequential assignments.
//!
//! # Multi-assign destructuring (§7)
//! Rules from docs/research/gen_docs/genexpr_language_reference.md §7:
//! - Extra LHS variables get 0.0 if the function returns fewer values
//! - Extra RHS values (return values) are ignored if LHS has fewer variables

use crate::ast::*;
use crate::lower::LowerError;
use std::collections::{HashMap, HashSet};

/// Validate that a function body has no early returns.
///
/// In M2, `return` is only legal as the sole/final top-level statement of the
/// function body. Any return nested inside If/While/DoWhile/For/Block, multiple
/// returns, or a return followed by trailing statements produces a clear error.
fn validate_early_returns(func: &FuncInfo) -> Result<(), LowerError> {
    let body = &func.body;
    for (i, stmt) in body.iter().enumerate() {
        let is_last = i == body.len() - 1;
        // Non-final return at top level → multiple returns / return-with-trailing
        if !is_last && is_or_contains_return(stmt) {
            return Err(LowerError {
                msg: format!(
                    "early return in function '{}': return must be the final statement of the function body",
                    func.name
                ),
                loc: Some(stmt.loc),
            });
        }
        // Any return nested inside If/While/DoWhile/For → early return
        if contains_nested_return_in_control_flow(stmt) {
            return Err(LowerError {
                msg: format!(
                    "early return in function '{}': return inside control-flow construct is not supported (M2 limitation)",
                    func.name
                ),
                loc: Some(stmt.loc),
            });
        }
    }
    Ok(())
}

/// True if the statement (or any recursively contained statement) is a Return.
fn is_or_contains_return(stmt: &Statement) -> bool {
    match &stmt.kind {
        StatementKind::Return(_) => true,
        StatementKind::Block(stmts) => stmts.iter().any(is_or_contains_return),
        StatementKind::If { then_branch, else_branch, .. } => {
            is_or_contains_return(then_branch)
                || else_branch.as_ref().is_some_and(|e| is_or_contains_return(e))
        }
        _ => false,
    }
}

/// True if a Return is nested inside a control-flow construct
/// (If/While/DoWhile/For), as opposed to being a bare top-level Return.
fn contains_nested_return_in_control_flow(stmt: &Statement) -> bool {
    match &stmt.kind {
        StatementKind::Block(stmts) => stmts.iter().any(is_or_contains_return),
        StatementKind::If { .. } | StatementKind::While { .. }
        | StatementKind::DoWhile { .. } | StatementKind::For { .. } => {
            // Any return inside these is nested (not bare top-level)
            has_return_deep(stmt)
        }
        StatementKind::Return(_) => false, // handled by the non-final check
        _ => false,
    }
}

/// Recursively check for any Return anywhere in the statement tree.
fn has_return_deep(stmt: &Statement) -> bool {
    match &stmt.kind {
        StatementKind::Return(_) => true,
        StatementKind::Block(stmts) => stmts.iter().any(has_return_deep),
        StatementKind::If { then_branch, else_branch, .. } => {
            has_return_deep(then_branch)
                || else_branch.as_ref().is_some_and(|e| has_return_deep(e))
        }
        StatementKind::While { body, .. } => has_return_deep(body),
        StatementKind::DoWhile { body, .. } => has_return_deep(body),
        StatementKind::For { body, .. } => has_return_deep(body),
        _ => false,
    }
}

/// Collect function declarations from a program, detect recursion, and inline
/// all user function calls. Modifies the program in place.
pub fn inline_functions(program: &mut Program) -> Result<(), LowerError> {
    // Phase 1: Extract function declarations
    let funcs = extract_funcs(program)?;
    if funcs.is_empty() {
        return Ok(());
    }

    // Phase 2: Validate no early returns
    for func in funcs.values() {
        validate_early_returns(func)?;
    }

    // Phase 3: Detect recursion
    detect_recursion(&funcs)?;

    // Phase 4: Inline function calls in program statements
    let mut counter = 0usize;
    let mut new_stmts: Vec<Statement> = Vec::new();
    for stmt in &program.statements {
        match &stmt.kind {
            StatementKind::FuncDecl { .. } => {
                // Skip — already extracted
            }
            _ => {
                let mut prologue = Vec::new();
                let transformed = inline_in_statement(stmt, &funcs, &mut counter, &mut prologue)?;
                new_stmts.extend(prologue);
                new_stmts.push(transformed);
            }
        }
    }
    program.statements = new_stmts;
    Ok(())
}

/// Information about a user-defined function.
struct FuncInfo {
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,
}

/// Result of inlining calls within an expression.
struct InlineResult {
    prologue: Vec<Statement>,
    expr: Expr,
}

/// Extract FuncDecl statements from the program, returning a name→func map.
fn extract_funcs(program: &Program) -> Result<HashMap<String, FuncInfo>, LowerError> {
    let mut funcs: HashMap<String, FuncInfo> = HashMap::new();
    for stmt in &program.statements {
        if let StatementKind::FuncDecl { name, params, body } = &stmt.kind {
            if funcs.contains_key(name) {
                return Err(LowerError {
                    msg: format!("duplicate function definition: {}", name),
                    loc: Some(stmt.loc),
                });
            }
            // Validate param names are unique
            let mut seen = HashSet::new();
            for p in params {
                if !seen.insert(p.clone()) {
                    return Err(LowerError {
                        msg: format!("duplicate parameter '{}' in function '{}'", p, name),
                        loc: Some(stmt.loc),
                    });
                }
            }
            funcs.insert(
                name.clone(),
                FuncInfo {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                },
            );
        }
    }
    Ok(funcs)
}

/// Detect cycles in the function call graph using DFS with three colors.
fn detect_recursion(funcs: &HashMap<String, FuncInfo>) -> Result<(), LowerError> {
    // Build adjacency: for each function, which other (user) functions does it call?
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, info) in funcs {
        let mut callees = Vec::new();
        collect_calls_in_stmts(&info.body, funcs, &mut callees);
        adj.insert(name.as_str(), callees);
    }

    // DFS with three colors: 0=white, 1=gray (in progress), 2=black (done)
    let mut color: HashMap<&str, u8> = HashMap::new();
    for name in funcs.keys() {
        color.insert(name.as_str(), 0);
    }
    let mut path: Vec<String> = Vec::new();

    for start in funcs.keys() {
        if color[start.as_str()] == 0 {
            if let Some(cycle) = dfs_cycle(start.as_str(), &adj, &mut color, &mut path) {
                return Err(LowerError {
                    msg: format!("recursion detected in function call graph: {}", cycle.join(" → ")),
                    loc: None,
                });
            }
        }
    }
    Ok(())
}

fn dfs_cycle<'a>(
    node: &'a str,
    adj: &HashMap<&'a str, Vec<&'a str>>,
    color: &mut HashMap<&'a str, u8>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    color.insert(node, 1);
    path.push(node.to_string());

    if let Some(callees) = adj.get(node) {
        for &callee in callees {
            match color.get(callee).copied().unwrap_or(0) {
                1 => {
                    // Found a back edge → cycle
                    let cycle_start = path.iter().position(|n| n == callee).unwrap();
                    let mut cycle: Vec<String> = path[cycle_start..].to_vec();
                    cycle.push(callee.to_string());
                    return Some(cycle);
                }
                0 => {
                    if let Some(result) = dfs_cycle(callee, adj, color, path) {
                        return Some(result);
                    }
                }
                _ => {}
            }
        }
    }

    color.insert(node, 2);
    path.pop();
    None
}

/// Collect user-function call names from a block of statements.
fn collect_calls_in_stmts<'a>(stmts: &'a [Statement], funcs: &HashMap<String, FuncInfo>, out: &mut Vec<&'a str>) {
    for stmt in stmts {
        collect_calls_in_stmt(stmt, funcs, out);
    }
}

fn collect_calls_in_stmt<'a>(stmt: &'a Statement, funcs: &HashMap<String, FuncInfo>, out: &mut Vec<&'a str>) {
    match &stmt.kind {
        StatementKind::Assign { expr, .. } | StatementKind::ExprStmt(expr) => {
            collect_calls_in_expr(expr, funcs, out);
        }
        StatementKind::If { cond, then_branch, else_branch } => {
            collect_calls_in_expr(cond, funcs, out);
            collect_calls_in_stmt(then_branch, funcs, out);
            if let Some(eb) = else_branch {
                collect_calls_in_stmt(eb, funcs, out);
            }
        }
        StatementKind::While { cond, body } => {
            collect_calls_in_expr(cond, funcs, out);
            collect_calls_in_stmt(body, funcs, out);
        }
        StatementKind::DoWhile { body, cond } => {
            collect_calls_in_stmt(body, funcs, out);
            collect_calls_in_expr(cond, funcs, out);
        }
        StatementKind::For { init, cond, step, body } => {
            if let Some(is) = init {
                collect_calls_in_stmt(is, funcs, out);
            }
            if let Some(c) = cond {
                collect_calls_in_expr(c, funcs, out);
            }
            if let Some(s) = step {
                collect_calls_in_expr(s, funcs, out);
            }
            collect_calls_in_stmt(body, funcs, out);
        }
        StatementKind::Block(stmts) => {
            collect_calls_in_stmts(stmts, funcs, out);
        }
        StatementKind::Return(exprs) => {
            for e in exprs {
                collect_calls_in_expr(e, funcs, out);
            }
        }
        StatementKind::MultiAssign { expr, .. } => {
            collect_calls_in_expr(expr, funcs, out);
        }
        _ => {}
    }
}

fn collect_calls_in_expr<'a>(expr: &'a Expr, funcs: &HashMap<String, FuncInfo>, out: &mut Vec<&'a str>) {
    match expr {
        Expr::Call { name, args, .. } => {
            if funcs.contains_key(name) {
                if !out.contains(&name.as_str()) {
                    out.push(name.as_str());
                }
            }
            for arg in args {
                collect_calls_in_expr(arg, funcs, out);
            }
        }
        Expr::BinOp { left, right, .. } => {
            collect_calls_in_expr(left, funcs, out);
            collect_calls_in_expr(right, funcs, out);
        }
        Expr::Unary(_, e) => {
            collect_calls_in_expr(e, funcs, out);
        }
        Expr::MemberCall { args, object, .. } => {
            collect_calls_in_expr(object, funcs, out);
            for arg in args {
                collect_calls_in_expr(arg, funcs, out);
            }
        }
        Expr::Ternary { cond, true_expr, false_expr } => {
            collect_calls_in_expr(cond, funcs, out);
            collect_calls_in_expr(true_expr, funcs, out);
            collect_calls_in_expr(false_expr, funcs, out);
        }
        Expr::Number(_) | Expr::Str(_) | Expr::Ident(_) => {}
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Inlining engine
// ═══════════════════════════════════════════════════════════════════

/// Process a statement, inlining any user function calls found in its expressions.
/// Returns the (possibly modified) statement. Prologue statements that must execute
/// BEFORE this statement are appended to `prologue`.
fn inline_in_statement(
    stmt: &Statement,
    funcs: &HashMap<String, FuncInfo>,
    counter: &mut usize,
    prologue: &mut Vec<Statement>,
) -> Result<Statement, LowerError> {
    match &stmt.kind {
        StatementKind::Assign { name, expr } => {
            let InlineResult { prologue: p, expr: new_expr } =
                inline_in_expr(expr, funcs, counter, stmt.loc)?;
            prologue.extend(p);
            Ok(Statement {
                kind: StatementKind::Assign {
                    name: name.clone(),
                    expr: new_expr,
                },
                loc: stmt.loc,
            })
        }
        StatementKind::If { cond, then_branch, else_branch } => {
            let InlineResult { prologue: p1, expr: new_cond } =
                inline_in_expr(cond, funcs, counter, stmt.loc)?;
            let mut then_prologue = Vec::new();
            let new_then = inline_in_statement(then_branch, funcs, counter, &mut then_prologue)?;
            let new_else = if let Some(eb) = else_branch {
                let mut else_prologue = Vec::new();
                let nb = inline_in_statement(eb, funcs, counter, &mut else_prologue)?;
                if else_prologue.is_empty() {
                    Some(Box::new(nb))
                } else {
                    let mut es: Vec<Statement> = else_prologue;
                    es.push(nb);
                    Some(Box::new(Statement { kind: StatementKind::Block(es), loc: stmt.loc }))
                }
            } else {
                None
            };

            let new_then = if then_prologue.is_empty() {
                new_then
            } else {
                let mut stmts: Vec<Statement> = then_prologue;
                stmts.push(new_then);
                Statement { kind: StatementKind::Block(stmts), loc: stmt.loc }
            };

            prologue.extend(p1);
            Ok(Statement {
                kind: StatementKind::If {
                    cond: new_cond,
                    then_branch: Box::new(new_then),
                    else_branch: new_else,
                },
                loc: stmt.loc,
            })
        }
        StatementKind::While { cond, body } => {
            let InlineResult { prologue: p, expr: new_cond } =
                inline_in_expr(cond, funcs, counter, stmt.loc)?;
            let mut body_prologue = Vec::new();
            let new_body = inline_in_statement(body, funcs, counter, &mut body_prologue)?;
            let new_body = if body_prologue.is_empty() {
                new_body
            } else {
                let mut stmts = body_prologue;
                stmts.push(new_body);
                Statement { kind: StatementKind::Block(stmts), loc: stmt.loc }
            };
            prologue.extend(p);
            Ok(Statement {
                kind: StatementKind::While { cond: new_cond, body: Box::new(new_body) },
                loc: stmt.loc,
            })
        }
        StatementKind::DoWhile { body, cond } => {
            let mut body_prologue = Vec::new();
            let new_body = inline_in_statement(body, funcs, counter, &mut body_prologue)?;
            let new_body = if body_prologue.is_empty() {
                new_body
            } else {
                let mut stmts = body_prologue;
                stmts.push(new_body);
                Statement { kind: StatementKind::Block(stmts), loc: stmt.loc }
            };
            let InlineResult { prologue: p, expr: new_cond } =
                inline_in_expr(cond, funcs, counter, stmt.loc)?;
            prologue.extend(p);
            Ok(Statement {
                kind: StatementKind::DoWhile { body: Box::new(new_body), cond: new_cond },
                loc: stmt.loc,
            })
        }
        StatementKind::For { init, cond, step, body } => {
            let new_init = if let Some(is) = init {
                let mut init_prologue = Vec::new();
                let ni = inline_in_statement(is, funcs, counter, &mut init_prologue)?;
                if init_prologue.is_empty() {
                    Some(Box::new(ni))
                } else {
                    let mut stmts = init_prologue;
                    stmts.push(ni);
                    Some(Box::new(Statement { kind: StatementKind::Block(stmts), loc: stmt.loc }))
                }
            } else {
                None
            };

            let new_cond = if let Some(c) = cond {
                let InlineResult { prologue: mut p, expr: nc } =
                    inline_in_expr(c, funcs, counter, stmt.loc)?;
                prologue.append(&mut p);
                Some(nc)
            } else {
                None
            };

            let new_step = if let Some(s) = step {
                let InlineResult { prologue: mut p, expr: ns } =
                    inline_in_expr(s, funcs, counter, stmt.loc)?;
                if !p.is_empty() {
                    prologue.append(&mut p);
                }
                Some(ns)
            } else {
                None
            };

            let mut body_prologue = Vec::new();
            let new_body = inline_in_statement(body, funcs, counter, &mut body_prologue)?;
            let new_body = if body_prologue.is_empty() {
                new_body
            } else {
                let mut stmts = body_prologue;
                stmts.push(new_body);
                Statement { kind: StatementKind::Block(stmts), loc: stmt.loc }
            };

            Ok(Statement {
                kind: StatementKind::For {
                    init: new_init, cond: new_cond, step: new_step, body: Box::new(new_body),
                },
                loc: stmt.loc,
            })
        }
        StatementKind::Block(stmts) => {
            let mut new_stmts = Vec::new();
            for s in stmts {
                let mut p = Vec::new();
                let ns = inline_in_statement(s, funcs, counter, &mut p)?;
                new_stmts.extend(p);
                new_stmts.push(ns);
            }
            Ok(Statement { kind: StatementKind::Block(new_stmts), loc: stmt.loc })
        }
        StatementKind::Return(_) => {
            Err(LowerError {
                msg: "return outside function definition".to_string(),
                loc: Some(stmt.loc),
            })
        }
        StatementKind::MultiAssign { names, expr } => {
            match expr {
                Expr::Call { name, args, named_args } if funcs.contains_key(name) => {
                    if !named_args.is_empty() {
                        return Err(LowerError {
                            msg: "named arguments in function calls not yet supported".to_string(),
                            loc: Some(stmt.loc),
                        });
                    }
                    handle_multi_assign_call(names, args, name, funcs, counter, stmt.loc, prologue)
                }
                _ => {
                    Err(LowerError {
                        msg: "multi-assign only supported for user function calls".to_string(),
                        loc: Some(stmt.loc),
                    })
                }
            }
        }
        StatementKind::Break | StatementKind::Continue => Ok(stmt.clone()),
        StatementKind::ExprStmt(expr) => {
            let InlineResult { prologue: p, expr: new_expr } =
                inline_in_expr(expr, funcs, counter, stmt.loc)?;
            prologue.extend(p);
            Ok(Statement { kind: StatementKind::ExprStmt(new_expr), loc: stmt.loc })
        }
        StatementKind::ParamDecl { .. } | StatementKind::Decl { .. } => Ok(stmt.clone()),
        StatementKind::Require(_) => Err(LowerError {
            msg: "require unsupported in M2".to_string(),
            loc: Some(stmt.loc),
        }),
        StatementKind::FuncDecl { .. } => Ok(stmt.clone()),
    }
}

/// Handle multi-assign where RHS is a user function call.
fn handle_multi_assign_call(
    names: &[String],
    args: &[Expr],
    func_name: &str,
    funcs: &HashMap<String, FuncInfo>,
    counter: &mut usize,
    loc: SourceLoc,
    prologue: &mut Vec<Statement>,
) -> Result<Statement, LowerError> {
    let func = &funcs[func_name];
    let instance_id = *counter;
    *counter += 1;

    // Inline args first
    let mut all_prologue = Vec::new();
    let mut new_args = Vec::new();
    for arg in args {
        let InlineResult { prologue: p, expr: a } =
            inline_in_expr(arg, funcs, counter, loc)?;
        all_prologue.extend(p);
        new_args.push(a);
    }

    // Generate param bindings with renamed names
    for (i, param_name) in func.params.iter().enumerate() {
        let arg_expr = if i < new_args.len() {
            new_args[i].clone()
        } else {
            Expr::Number(0.0)
        };
        all_prologue.push(Statement {
            kind: StatementKind::Assign {
                name: format!("__inl{}_{}", instance_id, param_name),
                expr: arg_expr,
            },
            loc,
        });
    }

    // Inline function body: rename locals and handle returns (with params)
    let renamed_body = inline_body_with_params(
        &func.body, &func.params, funcs, instance_id, counter, loc)?;
    all_prologue.extend(renamed_body);

    // Destructure return values into the multi-assign targets
    // §7 rules: extra LHS vars get 0.0, extra RHS values ignored
    let n_returns = count_returns(&func.body);
    for (i, target_name) in names.iter().enumerate() {
        if i < n_returns {
            all_prologue.push(Statement {
                kind: StatementKind::Assign {
                    name: target_name.clone(),
                    expr: Expr::Ident(format!("__inl{}_ret{}", instance_id, i)),
                },
                loc,
            });
        } else {
            // Extra LHS vars get 0 (§7 rule)
            all_prologue.push(Statement {
                kind: StatementKind::Assign {
                    name: target_name.clone(),
                    expr: Expr::Number(0.0),
                },
                loc,
            });
        }
    }

    prologue.extend(all_prologue);
    // Return a no-op — the multi-assign is fully consumed by the prologue
    Ok(Statement { kind: StatementKind::ExprStmt(Expr::Number(0.0)), loc })
}

/// Count how many return values a function produces (max of any return statement).
fn count_returns(body: &[Statement]) -> usize {
    for stmt in body {
        if let StatementKind::Return(exprs) = &stmt.kind {
            return exprs.len();
        }
        if let StatementKind::If { then_branch, else_branch, .. } = &stmt.kind {
            let then_count = count_returns_single(then_branch);
            let else_count = else_branch.as_ref().map(|e| count_returns_single(e)).unwrap_or(0);
            if then_count > 0 || else_count > 0 {
                return then_count.max(else_count);
            }
        }
    }
    1
}

fn count_returns_single(stmt: &Statement) -> usize {
    match &stmt.kind {
        StatementKind::Return(exprs) => exprs.len(),
        StatementKind::If { then_branch, else_branch, .. } => {
            let then_count = count_returns_single(then_branch);
            let else_count = else_branch.as_ref().map(|e| count_returns_single(e)).unwrap_or(0);
            then_count.max(else_count)
        }
        StatementKind::Block(stmts) => {
            for s in stmts.iter().rev() {
                let c = count_returns_single(s);
                if c > 0 {
                    return c;
                }
            }
            0
        }
        _ => 0,
    }
}

/// Recursively inline all user function calls in an expression tree.
/// Returns prologue statements and the modified expression.
fn inline_in_expr(
    expr: &Expr,
    funcs: &HashMap<String, FuncInfo>,
    counter: &mut usize,
    loc: SourceLoc,
) -> Result<InlineResult, LowerError> {
    match expr {
        Expr::Call { name, args, named_args } => {
            // First, recursively inline in all argument expressions
            let mut all_prologue = Vec::new();
            let mut new_args = Vec::new();
            for arg in args {
                let InlineResult { prologue, expr: a } =
                    inline_in_expr(arg, funcs, counter, loc)?;
                all_prologue.extend(prologue);
                new_args.push(a);
            }

            // Check if this is a user function call
            if funcs.contains_key(name) {
                if !named_args.is_empty() {
                    return Err(LowerError {
                        msg: "named arguments in function calls not yet supported".to_string(),
                        loc: Some(loc),
                    });
                }
                let func = &funcs[name];
                let instance_id = *counter;
                *counter += 1;

                // Param bindings: __inl<N>_<param> = <arg>
                for (i, param_name) in func.params.iter().enumerate() {
                    let arg_expr = if i < new_args.len() {
                        new_args[i].clone()
                    } else {
                        Expr::Number(0.0)
                    };
                    all_prologue.push(Statement {
                        kind: StatementKind::Assign {
                            name: format!("__inl{}_{}", instance_id, param_name),
                            expr: arg_expr,
                        },
                        loc,
                    });
                }

                // Inline function body (with param names for renaming)
                let renamed_body = inline_body_with_params(
                    &func.body, &func.params, funcs, instance_id, counter, loc)?;
                all_prologue.extend(renamed_body);

                // Replace call with return value identifier
                Ok(InlineResult {
                    prologue: all_prologue,
                    expr: Expr::Ident(format!("__inl{}_ret0", instance_id)),
                })
            } else {
                Ok(InlineResult {
                    prologue: all_prologue,
                    expr: Expr::Call {
                        name: name.clone(),
                        args: new_args,
                        named_args: named_args.clone(),
                    },
                })
            }
        }
        Expr::BinOp { op, left, right } => {
            let InlineResult { prologue: p1, expr: l } =
                inline_in_expr(left, funcs, counter, loc)?;
            let InlineResult { prologue: p2, expr: r } =
                inline_in_expr(right, funcs, counter, loc)?;
            let mut prologue = p1;
            prologue.extend(p2);
            Ok(InlineResult {
                prologue,
                expr: Expr::BinOp { op: *op, left: Box::new(l), right: Box::new(r) },
            })
        }
        Expr::Unary(op, e) => {
            let InlineResult { prologue, expr: inner } =
                inline_in_expr(e, funcs, counter, loc)?;
            Ok(InlineResult {
                prologue,
                expr: Expr::Unary(*op, Box::new(inner)),
            })
        }
        Expr::Ternary { cond, true_expr, false_expr } => {
            let InlineResult { prologue: p1, expr: c } =
                inline_in_expr(cond, funcs, counter, loc)?;
            let InlineResult { prologue: p2, expr: t } =
                inline_in_expr(true_expr, funcs, counter, loc)?;
            let InlineResult { prologue: p3, expr: f } =
                inline_in_expr(false_expr, funcs, counter, loc)?;
            let mut prologue = p1;
            prologue.extend(p2);
            prologue.extend(p3);
            Ok(InlineResult {
                prologue,
                expr: Expr::Ternary {
                    cond: Box::new(c), true_expr: Box::new(t), false_expr: Box::new(f),
                },
            })
        }
        Expr::MemberCall { object, method, args, named_args } => {
            let InlineResult { prologue: p1, expr: obj } =
                inline_in_expr(object, funcs, counter, loc)?;
            let mut all_prologue = p1;
            let mut new_args = Vec::new();
            for arg in args {
                let InlineResult { prologue, expr: a } =
                    inline_in_expr(arg, funcs, counter, loc)?;
                all_prologue.extend(prologue);
                new_args.push(a);
            }
            Ok(InlineResult {
                prologue: all_prologue,
                expr: Expr::MemberCall {
                    object: Box::new(obj),
                    method: method.clone(),
                    args: new_args,
                    named_args: named_args.clone(),
                },
            })
        }
        Expr::Number(_) | Expr::Str(_) | Expr::Ident(_) => Ok(InlineResult {
            prologue: Vec::new(),
            expr: expr.clone(),
        }),
    }
}

/// Inline the body of a function with renamed identifiers.
/// `params` is the list of parameter names that also need renaming.
fn inline_body_with_params(
    body: &[Statement],
    params: &[String],
    funcs: &HashMap<String, FuncInfo>,
    instance_id: usize,
    counter: &mut usize,
    loc: SourceLoc,
) -> Result<Vec<Statement>, LowerError> {
    let mut decl_names = collect_declared_names(body);
    for p in params {
        decl_names.insert(p.clone());
    }
    let mut result: Vec<Statement> = Vec::new();

    for stmt in body {
        match &stmt.kind {
            StatementKind::Return(exprs) => {
                // Convert return to __inl{id}_ret<N> assignments
                let mut all_prologue = Vec::new();
                let mut new_exprs = Vec::new();
                for e in exprs {
                    let InlineResult { prologue, expr: new_e } =
                        inline_in_expr_with_rename(e, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                    all_prologue.extend(prologue);
                    new_exprs.push(new_e);
                }
                result.extend(all_prologue);
                for (i, e) in new_exprs.iter().enumerate() {
                    result.push(Statement {
                        kind: StatementKind::Assign {
                            name: format!("__inl{}_ret{}", instance_id, i),
                            expr: e.clone(),
                        },
                        loc: stmt.loc,
                    });
                }
            }
            StatementKind::Assign { name, expr } => {
                let new_name = if decl_names.contains(name) {
                    format!("__inl{}_{}", instance_id, name)
                } else {
                    name.clone()
                };
                let InlineResult { prologue, expr: new_expr } =
                    inline_in_expr_with_rename(expr, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                result.extend(prologue);
                result.push(Statement {
                    kind: StatementKind::Assign { name: new_name, expr: new_expr },
                    loc: stmt.loc,
                });
            }
            StatementKind::Decl { ty, items } => {
                let new_items: Vec<Declarator> = items
                    .iter()
                    .map(|item| {
                        let new_args: Vec<Expr> = item
                            .args
                            .iter()
                            .map(|a| {
                                inline_in_expr_with_rename(a, funcs, counter, instance_id, &decl_names, loc)
                                    .unwrap_or_else(|_| InlineResult { prologue: vec![], expr: a.clone() })
                                    .expr
                            })
                            .collect();
                        Declarator {
                            name: format!("__inl{}_{}", instance_id, item.name),
                            args: new_args,
                            named_args: item.named_args.clone(),
                        }
                    })
                    .collect();
                result.push(Statement {
                    kind: StatementKind::Decl { ty: *ty, items: new_items },
                    loc: stmt.loc,
                });
            }
            StatementKind::If { cond, then_branch, else_branch } => {
                let InlineResult { prologue: p_cond, expr: new_cond } =
                    inline_in_expr_with_rename(cond, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                let new_then = inline_body_single_with_params(then_branch, params, funcs, instance_id, counter, stmt.loc)?;
                let new_else = if let Some(eb) = else_branch {
                    Some(Box::new(inline_body_single_with_params(eb, params, funcs, instance_id, counter, stmt.loc)?))
                } else {
                    None
                };
                let mut stmts: Vec<Statement> = p_cond;
                stmts.push(Statement {
                    kind: StatementKind::If {
                        cond: new_cond, then_branch: Box::new(new_then), else_branch: new_else,
                    },
                    loc: stmt.loc,
                });
                result.extend(stmts);
            }
            StatementKind::While { cond, body } => {
                let InlineResult { prologue: p_cond, expr: new_cond } =
                    inline_in_expr_with_rename(cond, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                let new_body = inline_body_single_with_params(body, params, funcs, instance_id, counter, stmt.loc)?;
                result.extend(p_cond);
                result.push(Statement {
                    kind: StatementKind::While { cond: new_cond, body: Box::new(new_body) },
                    loc: stmt.loc,
                });
            }
            StatementKind::DoWhile { body, cond } => {
                let new_body = inline_body_single_with_params(body, params, funcs, instance_id, counter, stmt.loc)?;
                let InlineResult { prologue: p_cond, expr: new_cond } =
                    inline_in_expr_with_rename(cond, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                result.extend(p_cond);
                result.push(Statement {
                    kind: StatementKind::DoWhile { body: Box::new(new_body), cond: new_cond },
                    loc: stmt.loc,
                });
            }
            StatementKind::For { init, cond, step, body } => {
                let new_init = if let Some(is) = init {
                    Some(Box::new(inline_body_single_with_params(is, params, funcs, instance_id, counter, stmt.loc)?))
                } else {
                    None
                };
                let new_cond = if let Some(c) = cond {
                    let InlineResult { prologue: _, expr: nc } =
                        inline_in_expr_with_rename(c, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                    Some(nc)
                } else {
                    None
                };
                let new_step = if let Some(s) = step {
                    let InlineResult { prologue: _, expr: ns } =
                        inline_in_expr_with_rename(s, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                    Some(ns)
                } else {
                    None
                };
                let new_body = inline_body_single_with_params(body, params, funcs, instance_id, counter, stmt.loc)?;
                result.push(Statement {
                    kind: StatementKind::For {
                        init: new_init, cond: new_cond, step: new_step, body: Box::new(new_body),
                    },
                    loc: stmt.loc,
                });
            }
            StatementKind::Block(stmts) => {
                let new_stmts = inline_body_with_params(stmts, params, funcs, instance_id, counter, loc)?;
                result.push(Statement { kind: StatementKind::Block(new_stmts), loc: stmt.loc });
            }
            StatementKind::Break | StatementKind::Continue => {
                result.push(stmt.clone());
            }
            StatementKind::ExprStmt(expr) => {
                let InlineResult { prologue, expr: new_expr } =
                    inline_in_expr_with_rename(expr, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                result.extend(prologue);
                result.push(Statement { kind: StatementKind::ExprStmt(new_expr), loc: stmt.loc });
            }
            StatementKind::MultiAssign { names, expr: rhs } => {
                match rhs {
                    Expr::Call { name: callee_name, args, named_args } if funcs.contains_key(callee_name) => {
                        if !named_args.is_empty() {
                            return Err(LowerError {
                                msg: "named arguments in function calls not yet supported".to_string(),
                                loc: Some(stmt.loc),
                            });
                        }
                        let callee_func = &funcs[callee_name];
                        let callee_id = *counter;
                        *counter += 1;
                        let mut all_prologue = Vec::new();
                        let mut new_args = Vec::new();
                        for arg in args {
                            let InlineResult { prologue, expr: a } =
                                inline_in_expr_with_rename(arg, funcs, counter, instance_id, &decl_names, stmt.loc)?;
                            all_prologue.extend(prologue);
                            new_args.push(a);
                        }
                        for (i, param_name) in callee_func.params.iter().enumerate() {
                            let arg_expr = if i < new_args.len() { new_args[i].clone() } else { Expr::Number(0.0) };
                            all_prologue.push(Statement {
                                kind: StatementKind::Assign {
                                    name: format!("__inl{}_{}", callee_id, param_name), expr: arg_expr,
                                },
                                loc: stmt.loc,
                            });
                        }
                        let renamed_body = inline_body_with_params(
                            &callee_func.body, &callee_func.params, funcs, callee_id, counter, stmt.loc)?;
                        all_prologue.extend(renamed_body);
                        let n_returns = count_returns(&callee_func.body);
                        for (i, target_name) in names.iter().enumerate() {
                            let new_target = if decl_names.contains(target_name) {
                                format!("__inl{}_{}", instance_id, target_name)
                            } else {
                                target_name.clone()
                            };
                            if i < n_returns {
                                all_prologue.push(Statement {
                                    kind: StatementKind::Assign {
                                        name: new_target,
                                        expr: Expr::Ident(format!("__inl{}_ret{}", callee_id, i)),
                                    },
                                    loc: stmt.loc,
                                });
                            } else {
                                // Extra LHS vars get 0 (§7)
                                all_prologue.push(Statement {
                                    kind: StatementKind::Assign {
                                        name: new_target,
                                        expr: Expr::Number(0.0),
                                    },
                                    loc: stmt.loc,
                                });
                            }
                        }
                        result.extend(all_prologue);
                    }
                    _ => {
                        return Err(LowerError {
                            msg: "multi-assign only supported for user function calls".to_string(),
                            loc: Some(stmt.loc),
                        });
                    }
                }
            }
            StatementKind::ParamDecl { name, default } => {
                result.push(Statement {
                    kind: StatementKind::ParamDecl {
                        name: format!("__inl{}_{}", instance_id, name), default: *default,
                    },
                    loc: stmt.loc,
                });
            }
            StatementKind::Require(_) => {
                return Err(LowerError {
                    msg: "require unsupported in M2".to_string(),
                    loc: Some(stmt.loc),
                });
            }
            StatementKind::FuncDecl { .. } => {
                return Err(LowerError {
                    msg: "nested function declarations not supported".to_string(),
                    loc: Some(stmt.loc),
                });
            }
        }
    }

    Ok(result)
}

/// Inline a single statement body (for if/while/for bodies) with identifier rename.
/// `params` are the function parameters that also need renaming.
fn inline_body_single_with_params(
    stmt: &Statement,
    params: &[String],
    funcs: &HashMap<String, FuncInfo>,
    instance_id: usize,
    counter: &mut usize,
    loc: SourceLoc,
) -> Result<Statement, LowerError> {
    let stmts = inline_body_with_params(
        &[stmt.clone()], params, funcs, instance_id, counter, loc)?;
    if stmts.len() == 1 {
        Ok(stmts.into_iter().next().unwrap())
    } else {
        Ok(Statement { kind: StatementKind::Block(stmts), loc })
    }
}

/// Inline calls in an expression AND rename identifiers that match declared names.
fn inline_in_expr_with_rename(
    expr: &Expr,
    funcs: &HashMap<String, FuncInfo>,
    counter: &mut usize,
    instance_id: usize,
    decl_names: &HashSet<String>,
    loc: SourceLoc,
) -> Result<InlineResult, LowerError> {
    let InlineResult { prologue, expr: inlined_expr } =
        inline_in_expr(expr, funcs, counter, loc)?;
    let renamed_expr = rename_ident_expr(&inlined_expr, instance_id, decl_names);
    Ok(InlineResult { prologue, expr: renamed_expr })
}

/// Rename identifiers in an expression tree.
fn rename_ident_expr(expr: &Expr, instance_id: usize, decl_names: &HashSet<String>) -> Expr {
    match expr {
        Expr::Ident(name) => {
            if decl_names.contains(name) {
                Expr::Ident(format!("__inl{}_{}", instance_id, name))
            } else {
                Expr::Ident(name.clone())
            }
        }
        Expr::BinOp { op, left, right } => Expr::BinOp {
            op: *op,
            left: Box::new(rename_ident_expr(left, instance_id, decl_names)),
            right: Box::new(rename_ident_expr(right, instance_id, decl_names)),
        },
        Expr::Unary(op, e) => Expr::Unary(*op, Box::new(rename_ident_expr(e, instance_id, decl_names))),
        Expr::Call { name, args, named_args } => Expr::Call {
            name: name.clone(),
            args: args.iter().map(|a| rename_ident_expr(a, instance_id, decl_names)).collect(),
            named_args: named_args.iter().map(|(k, v)| (k.clone(), rename_ident_expr(v, instance_id, decl_names))).collect(),
        },
        Expr::MemberCall { object, method, args, named_args } => Expr::MemberCall {
            object: Box::new(rename_ident_expr(object, instance_id, decl_names)),
            method: method.clone(),
            args: args.iter().map(|a| rename_ident_expr(a, instance_id, decl_names)).collect(),
            named_args: named_args.iter().map(|(k, v)| (k.clone(), rename_ident_expr(v, instance_id, decl_names))).collect(),
        },
        Expr::Ternary { cond, true_expr, false_expr } => Expr::Ternary {
            cond: Box::new(rename_ident_expr(cond, instance_id, decl_names)),
            true_expr: Box::new(rename_ident_expr(true_expr, instance_id, decl_names)),
            false_expr: Box::new(rename_ident_expr(false_expr, instance_id, decl_names)),
        },
        Expr::Number(_) | Expr::Str(_) => expr.clone(),
    }
}

/// Collect names declared in a function body (params + History/Data/Buffer/Delay + assigned locals).
fn collect_declared_names(body: &[Statement]) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_names_in_stmts(body, &mut names);
    names
}

fn collect_names_in_stmts(stmts: &[Statement], names: &mut HashSet<String>) {
    for stmt in stmts {
        collect_names_in_stmt(stmt, names);
    }
}

fn collect_names_in_stmt(stmt: &Statement, names: &mut HashSet<String>) {
    match &stmt.kind {
        StatementKind::Assign { name, .. } | StatementKind::ParamDecl { name, .. } => {
            names.insert(name.clone());
        }
        StatementKind::Decl { items, .. } => {
            for item in items {
                names.insert(item.name.clone());
            }
        }
        StatementKind::If { then_branch, else_branch, .. } => {
            collect_names_in_stmt(then_branch, names);
            if let Some(eb) = else_branch {
                collect_names_in_stmt(eb, names);
            }
        }
        StatementKind::While { body, .. } => collect_names_in_stmt(body, names),
        StatementKind::DoWhile { body, .. } => collect_names_in_stmt(body, names),
        StatementKind::For { init, body, .. } => {
            if let Some(is) = init {
                collect_names_in_stmt(is, names);
            }
            collect_names_in_stmt(body, names);
        }
        StatementKind::Block(stmts) => collect_names_in_stmts(stmts, names),
        _ => {}
    }
}
