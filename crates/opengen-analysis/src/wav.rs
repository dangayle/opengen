//! WAV file I/O and golden-file test macros

use std::path::Path;

/// Write samples to a WAV file (f32 PCM format).
pub fn write_wav(path: &Path, samples: &[f64], sr: u32) -> std::io::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sr,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    
    let mut writer = hound::WavWriter::create(path, spec)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    for &sample in samples {
        writer.write_sample(sample as f32)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    writer.finalize()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

/// Read a WAV file and return samples as f64 and the sample rate.
/// Supports both f32 and i16 formats, converting to f64.
pub fn read_wav(path: &Path) -> std::io::Result<(Vec<f64>, u32)> {
    let mut reader = hound::WavReader::open(path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let spec = reader.spec();
    let sr = spec.sample_rate;
    
    let samples: Vec<f64> = match spec.sample_format {
        hound::SampleFormat::Float => {
            reader.samples::<f32>()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
                .into_iter()
                .map(|s| s as f64)
                .collect()
        }
        hound::SampleFormat::Int => {
            // Assume 16-bit int
            reader.samples::<i16>()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
                .into_iter()
                .map(|s| s as f64 / 32768.0)
                .collect()
        }
    };
    
    Ok((samples, sr))
}

/// Assert that rendering `src` matches a golden WAV file sample-by-sample within `tol`.
/// 
/// - If the golden file doesn't exist, the test SKIPs (prints note, returns ok).
/// - If env var OPENGEN_BLESS=1, writes/overwrites the golden file and passes.
/// - Otherwise, compares rendered output to golden file within tolerance.
#[macro_export]
macro_rules! assert_render_matches {
    ($src:expr, $golden_path:expr, $tol:expr) => {{
        use std::path::Path;
        
        let golden_path = Path::new($golden_path);
        
        // Check if we're in bless mode
        let bless = std::env::var("OPENGEN_BLESS").is_ok();
        
        // Render the source at 48 kHz for 1 second (48000 samples)
        let render = opengen_testkit::render($src, 48_000.0, 48_000);
        let samples: Vec<f64> = render.ch(0).to_vec();
        
        if bless {
            // Write/overwrite the golden file
            $crate::wav::write_wav(golden_path, &samples, 48_000)
                .expect("failed to write golden file");
            eprintln!("✓ Blessed golden file: {}", golden_path.display());
        } else if !golden_path.exists() {
            // Skip test if golden file doesn't exist
            eprintln!("⊘ Skipping test: golden file not found: {}", golden_path.display());
            eprintln!("  Run with OPENGEN_BLESS=1 to create it.");
        } else {
            // Compare to golden file
            let (golden, _sr) = $crate::wav::read_wav(golden_path)
                .expect("failed to read golden file");
            
            assert_eq!(samples.len(), golden.len(), 
                "sample count mismatch: got {} vs golden {}",
                samples.len(), golden.len());
            
            for (i, (&s, &g)) in samples.iter().zip(golden.iter()).enumerate() {
                assert!((s - g).abs() <= $tol,
                    "sample {} differs: got {} vs golden {} (diff {})",
                    i, s, g, (s - g).abs());
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn wav_roundtrip() {
        // Test write/read roundtrip using temp_dir
        let temp_dir = env::temp_dir();
        let path = temp_dir.join("opengen_test_roundtrip.wav");
        
        // Create test samples
        let samples: Vec<f64> = (0..100).map(|i| (i as f64 / 100.0) * 2.0 - 1.0).collect();
        let sr = 48_000;
        
        // Write
        write_wav(&path, &samples, sr).expect("write failed");
        
        // Read back
        let (read_samples, read_sr) = read_wav(&path).expect("read failed");
        
        // Verify
        assert_eq!(read_sr, sr);
        assert_eq!(read_samples.len(), samples.len());
        
        for (i, (&original, &read_back)) in samples.iter().zip(read_samples.iter()).enumerate() {
            // f32 precision may introduce small errors
            assert!((original - read_back).abs() < 1e-6,
                "sample {} differs: original {} vs read_back {}",
                i, original, read_back);
        }
        
        // Clean up
        std::fs::remove_file(&path).ok();
    }
}
