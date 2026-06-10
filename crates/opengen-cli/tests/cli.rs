//! CLI smoke tests - verify the binary works end-to-end

use std::process::Command;
use std::path::Path;
use std::env;

fn fixture_path(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

#[test]
fn run_subcommand_prints_stats() {
    let output = Command::new(env!("CARGO_BIN_EXE_opengen"))
        .args(&["run", &fixture_path("counter.genexpr"), "--samples", "100"])
        .output()
        .expect("failed to execute");
    
    assert!(output.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should print stats like "samples", "min", "max", "rms", "dc"
    assert!(stdout.contains("samples") || stdout.contains("rms") || stdout.contains("min"),
        "stdout should contain stats keywords, got: {}", stdout);
}

#[test]
fn probe_subcommand_prints_values() {
    let output = Command::new(env!("CARGO_BIN_EXE_opengen"))
        .args(&[
            "probe", 
            &fixture_path("counter.genexpr"), 
            "--tap", "h", 
            "--samples", "3"
        ])
        .output()
        .expect("failed to execute");
    
    assert!(output.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse lines as numbers - counter starts at 0, so produces 0, 1, 2
    let values: Vec<f64> = stdout.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse::<f64>().expect("line should be a number"))
        .collect();
    
    assert_eq!(values.len(), 3, "should have 3 values");
    assert_eq!(values[0], 0.0, "first value should be 0");
    assert_eq!(values[1], 1.0, "second value should be 1");
    assert_eq!(values[2], 2.0, "third value should be 2");
}

#[test]
fn plot_subcommand_creates_svg() {
    let temp_dir = env::temp_dir();
    let svg_path = temp_dir.join("opengen_test_plot.svg");
    
    // Remove if exists from previous run
    std::fs::remove_file(&svg_path).ok();
    
    let output = Command::new(env!("CARGO_BIN_EXE_opengen"))
        .args(&[
            "plot",
            &fixture_path("counter.genexpr"),
            "--response",
            svg_path.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute");
    
    assert!(output.status.success(), "exit code should be 0");
    
    // Check that the SVG file was created and is non-empty
    assert!(svg_path.exists(), "SVG file should exist");
    let content = std::fs::read_to_string(&svg_path)
        .expect("should be able to read SVG");
    assert!(content.len() > 0, "SVG should not be empty");
    assert!(content.contains("<svg") || content.contains("svg"),
        "should contain SVG content");
    
    // Clean up
    std::fs::remove_file(&svg_path).ok();
}
