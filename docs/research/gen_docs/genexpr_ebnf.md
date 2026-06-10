# GenExpr EBNF Grammar

Derived from the official `genexpr.pegjs` parser grammar (extracted from `@rnbo/genexpr_js` v1.4.2).

Uses ISO EBNF notation:

| Symbol | Meaning |
|--------|---------|
| `=`    | is defined as |
| `,`    | followed by (sequence) |
| `\|`   | or (alternation) |
| `{ }` | zero or more repetitions |
| `[ ]` | optional (zero or one) |
| `( )` | grouping |
| `" "` | literal string |
| `(* *)` | comment |
| `;`   | end of rule |

---

## Top-Level Structure

```ebnf
translation_unit = { compiler_command }
                 , { function_declaration }
                 , { declaration }
                 , body_statement_list ;

(* A single expression, used by inline [expr] boxes.
   Implicitly assigns to out1. No semicolon needed. *)
gen_expression = expression_list ;
```

---

## Compiler Commands

```ebnf
(* Currently only "require" exists. *)
compiler_command = "require" , ( string_literal | "(" , string_literal , ")" ) , [ ";" ] ;
```

---

## Function Declarations

```ebnf
(* Must appear before declarations and body statements. No nesting allowed. *)
function_declaration = identifier
                     , "(" , [ parameter_list ] , ")"
                     , "{" , { declaration } , [ function_body ] , "}" ;

parameter_list = parameter , { "," , parameter } ;

(* Parameters may have default values *)
parameter = identifier , "=" , expression
          | identifier ;

function_body = { statement } , [ jump_statement ] ;
```

---

## Declarations

```ebnf
(* Typed state object declarations. Types: History, Delay, Data, Param, Buffer, etc. *)
declaration = type_name , declarator , { "," , declarator } , ";" ;

type_name = identifier ;   (* History | Delay | Data | Param | Buffer | ... *)

(* Optional initializer is a call-style argument list *)
declarator = identifier , [ "(" , [ argument_list ] , ")" ] ;
```

---

## Statements

```ebnf
(* Body of translation unit: allows inline declarations *)
body_statement_list = { body_statement } , [ jump_statement ] ;

body_statement = compound_statement
               | expression_statement
               | if_statement
               | iteration_statement
               | declaration ;            (* inline declarations allowed here *)

(* Body of function: no inline declarations *)
statement_list = { statement } , [ jump_statement ] ;

statement = compound_statement
          | expression_statement
          | if_statement
          | iteration_statement ;

compound_statement = "{" , statement_list , "}" ;

expression_statement = ";"
                     | expression , ";" ;

(* if / else if / else *)
if_statement = "if" , "(" , expression , ")" , statement
             , [ "else" , statement ] ;

iteration_statement = while_statement
                    | do_statement
                    | for_statement ;

while_statement = "while" , "(" , expression , ")" , statement ;

do_statement = "do" , statement , "while" , "(" , expression , ")" , ";" ;

(* for() init must be a variable declaration (not a pre-existing variable) *)
for_statement = "for" , "("
              , multi_decl_expression , ";"
              , expression , ";"
              , [ expression ]
              , ")" , statement ;

(* Multiple variables in a for() init: for (i, j = 0, 0; ...) *)
multi_decl_expression = identifier , { "," , identifier }
                       , "=" , expression , { "," , expression } ;

jump_statement = continue_statement
               | break_statement
               | return_statement ;

continue_statement = "continue" , ";" ;
break_statement    = "break" , ";" ;

(* Single or multiple return values *)
return_statement = "return" , expression_list , ";"
                 | "return" , ";" ;
```

---

## Expressions

```ebnf
expression = assignment_expression ;

(* Multiple assignment: a, b = expr1, expr2 *)
assignment_expression = lhs_list , assignment_op , expression_list
                      | conditional_expression ;

lhs_list = unary_expression , { "," , unary_expression } ;

(* Comma-separated list of values (for MRV and parallel assignment) *)
expression_list = expression , { "," , expression } ;

assignment_op = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;

(* Ternary operator *)
conditional_expression = binary_expression , "?" , expression , ":" , expression ;

(* Binary operators with precedence (listed highest to lowest binding):
     1: * / %
     2: + -
     3: << >>
     4: <= >= < > <=p >=p <p >p
     5: == != ==p !=p
     6: &
     7: ^
     8: |
     9: &&
    10: ^^
    11: ||
*)
binary_expression = unary_expression , { binary_op , unary_expression } ;

binary_op = "<=p" | ">=p" | "<p"  | ">p"
          | "<="  | ">="  | "<<"  | ">>"
          | "==p" | "!=p" | "=="  | "!="
          | "&&"  | "||"  | "^^"
          | "+"   | "-"   | "*"   | "/"
          | "%"   | "^"   | "&"   | "|"
          | "<"   | ">" ;

(* Unary prefix operators *)
unary_expression = postfix_expression
                 | unary_op , unary_expression ;

unary_op = "-" | "!" | "~" ;

(* Postfix: member access, subscript, function call *)
postfix_expression = primary_expression , { postfix_op } ;

postfix_op = "[" , expression , "]"              (* subscript:     obj[i]     *)
           | "." , identifier                     (* member access: obj.field  *)
           | "(" , [ argument_list ] , ")" ;      (* call:          func(args) *)

argument_list = argument , { "," , argument } ;

(* Arguments may be positional or named (attribute-style) *)
argument = identifier , "=" , expression          (* named:    interp="linear" *)
         | expression ;                            (* positional               *)

primary_expression = identifier
                   | literal
                   | "(" , expression , ")" ;
```

---

## Identifiers

```ebnf
identifier = ( letter | "_" ) , { letter | digit | "_" } ;

(* Reserved words. Cannot be used as identifiers. *)
reserved_word = "return" | "continue" | "break"
              | "true"   | "false"    | "null" ;

letter = "A" | "B" | ... | "Z" | "a" | "b" | ... | "z" ;
digit  = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
```

---

## Literals

```ebnf
literal = integer_literal
        | float_literal
        | bool_literal
        | null_literal
        | string_literal ;

(* Integers: decimal or hexadecimal *)
integer_literal = hex_integer | decimal_integer ;

hex_integer     = "0x" , hex_digit , { hex_digit } ;
decimal_integer = "0"
                | non_zero_digit , { digit } ;

non_zero_digit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
hex_digit      = digit | "a" | ... | "f" | "A" | ... | "F" ;

(* Floats: must have decimal point or exponent to distinguish from integer *)
float_literal = "." , digit , { digit } , [ exponent ]
              | decimal_integer , "." , { digit } , [ exponent ]
              | decimal_integer , exponent ;

exponent = ( "e" | "E" ) , [ "+" | "-" ] , digit , { digit } ;

bool_literal = "true" | "false" ;

null_literal = "null" ;

(* Simple strings: no escape sequences, no multiline *)
string_literal = '"' , { any_char_except_double_quote } , '"'
               | "'" , { any_char_except_single_quote } , "'" ;
```

---

## Whitespace & Comments

```ebnf
(* Whitespace and comments are ignored between all tokens *)
whitespace = { " " | "\t" | "\n" | "\r" | comment } ;

comment = single_line_comment | multi_line_comment ;

single_line_comment = "//" , { any_char_except_newline } , newline ;
multi_line_comment  = "/*" , { any_char } , "*/" ;
```

---

## Key Semantic Constraints

These are not expressible in EBNF but are enforced by the compiler:

- **Order**: `compiler_command`s → `function_declaration`s → `declaration`s → body statements. This ordering is strict.
- **No nested functions**: `function_declaration` cannot appear inside another function body.
- **Reserved words** (`return`, `break`, `continue`, `true`, `false`, `null`) cannot be used as identifiers.
- **Type names** in declarations (`History`, `Delay`, `Data`, `Param`, `Buffer`) are identifiers, not keywords. The compiler interprets them contextually.
- **Multiple return values**: `return a, b;` is valid only inside a function. The caller must destructure: `x, y = myFunc(z);`.
- **Assignment in expressions**: `a = b` is an expression (not just a statement), so `if (x = foo()) { }` is syntactically valid.
- **Implicit `out1`**: In `gen_expression` mode (single expression, no assignment), the result is implicitly assigned to `out1`.
