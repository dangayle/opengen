//! OpenGen CLI - generate, analyze, and probe audio patches

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "opengen")]
#[command(about = "OpenGen audio synthesis language CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a patch to audio or stats
    Run {
        /// Path to .genexpr patch file
        patch: PathBuf,
        
        /// Sample rate in Hz
        #[arg(long, default_value = "48000")]
        sr: u32,
        
        /// Number of samples to generate
        #[arg(long, default_value = "48000")]
        samples: usize,
        
        /// Optional WAV output file path (if omitted, prints stats only)
        #[arg(long)]
        wav: Option<PathBuf>,
    },
    
    /// Plot frequency response to SVG
    Plot {
        /// Path to .genexpr patch file
        patch: PathBuf,
        
        /// Sample rate in Hz
        #[arg(long, default_value = "48000")]
        sr: u32,
        
        /// Output SVG file path
        #[arg(long)]
        response: PathBuf,
    },
    
    /// Probe internal signal values
    Probe {
        /// Path to .genexpr patch file
        patch: PathBuf,
        
        /// Sample rate in Hz
        #[arg(long, default_value = "48000")]
        sr: u32,
        
        /// Signal name to tap
        #[arg(long)]
        tap: String,
        
        /// Number of samples to probe
        #[arg(long, default_value = "100")]
        samples: usize,
    },

    /// Emit C++ source from a patch
    Emit {
        /// Path to .genexpr patch file
        patch: PathBuf,

        /// Sample rate in Hz
        #[arg(long, default_value = "48000")]
        sr: u32,

        /// Output directory (default: current dir)
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Run { patch, sr, samples, wav } => {
            run_command(patch, sr as f64, samples, wav)
        }
        Commands::Plot { patch, sr, response } => {
            plot_command(patch, sr as f64, response)
        }
        Commands::Probe { patch, sr, tap, samples } => {
            probe_command(patch, sr as f64, tap, samples)
        }
        Commands::Emit { patch, sr, output } => {
            emit_command(patch, sr as f64, output)
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_command(
    patch_path: PathBuf,
    sr: f64,
    n_samples: usize,
    wav_path: Option<PathBuf>,
) -> Result<(), String> {
    // Read patch source
    let src = std::fs::read_to_string(&patch_path)
        .map_err(|e| format!("failed to read {}: {}", patch_path.display(), e))?;
    
    // Render via testkit (handles zero inputs)
    let render = opengen_testkit::render(&src, sr, n_samples);
    
    // Process channel 0 (M1: single channel stats)
    let samples = render.ch(0);
    
    if samples.is_empty() {
        return Err("no output samples".to_string());
    }
    
    // Compute stats
    let min = samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let rms = (samples.iter().map(|x| x * x).sum::<f64>() / samples.len() as f64).sqrt();
    
    println!("samples={}, min={:.6}, max={:.6}, dc={:.6}, rms={:.6}",
             samples.len(), min, max, mean, rms);
    
    // Write WAV if requested (channel 0 only)
    if let Some(wav_path) = wav_path {
        let samples = render.ch(0);
        opengen_analysis::wav::write_wav(&wav_path, samples, sr as u32)
            .map_err(|e| format!("failed to write WAV: {}", e))?;
        println!("Wrote {}", wav_path.display());
    }
    
    Ok(())
}

fn plot_command(
    patch_path: PathBuf,
    sr: f64,
    svg_path: PathBuf,
) -> Result<(), String> {
    // Read patch source
    let src = std::fs::read_to_string(&patch_path)
        .map_err(|e| format!("failed to read {}: {}", patch_path.display(), e))?;
    
    // Compute frequency response
    let response = opengen_analysis::freq_response(&src, sr, 8192);
    
    // Plot to SVG
    opengen_analysis::plot::plot_response_svg(&response, &svg_path, 800, 600)
        .map_err(|e| format!("failed to plot: {}", e))?;
    
    println!("Wrote {}", svg_path.display());
    
    Ok(())
}

fn probe_command(
    patch_path: PathBuf,
    sr: f64,
    tap_name: String,
    n_samples: usize,
) -> Result<(), String> {
    // Read patch source
    let src = std::fs::read_to_string(&patch_path)
        .map_err(|e| format!("failed to read {}: {}", patch_path.display(), e))?;
    
    // Parse and compile with probes
    let graph = opengen_genexpr::parse_and_lower(&src)
        .map_err(|e| format!("parse error: {}", e))?;
    
    let mut patch = opengen_compile::compile_with_probes(
        &graph,
        &opengen_ops::Registry::core(),
        sr,
        &[tap_name.as_str()],
    ).map_err(|e| format!("compile error: {}", e.0))?;
    
    for _ in 0..n_samples {
        patch.process(&[]);
        
        // Read probe value
        if let Some(values) = patch.probe(&tap_name) {
            if let Some(&value) = values.last() {
                println!("{}", value);
            }
        } else {
            return Err(format!("tap '{}' not found", tap_name));
        }
    }
    
    Ok(())
}

fn emit_command(
    patch_path: PathBuf,
    sr: f64,
    output_dir: Option<PathBuf>,
) -> Result<(), String> {
    let src = std::fs::read_to_string(&patch_path)
        .map_err(|e| format!("failed to read {}: {}", patch_path.display(), e))?;

    let graph = opengen_genexpr::parse_and_lower(&src)
        .map_err(|e| format!("parse error: {}", e))?;

    let cpp = opengen_emit_cpp::emit_cpp(&graph, &opengen_ops::Registry::core(), sr)
        .map_err(|e| format!("emit error: {}", e))?;

    let out_dir = output_dir.unwrap_or_else(|| PathBuf::from("."));
    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("cannot create output dir: {}", e))?;

    let header_path = out_dir.join("opengen_patch.h");
    let body_path = out_dir.join("opengen_patch.cpp");

    std::fs::write(&header_path, &cpp.header)
        .map_err(|e| format!("failed to write {}: {}", header_path.display(), e))?;
    std::fs::write(&body_path, &cpp.body)
        .map_err(|e| format!("failed to write {}: {}", body_path.display(), e))?;

    println!("Emitted {}", header_path.display());
    println!("Emitted {}", body_path.display());
    Ok(())
}
