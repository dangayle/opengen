//! SVG plotting helpers for frequency response visualization

use crate::Response;
use plotters::prelude::*;
use std::path::Path;

/// Plot a frequency response to an SVG file.
///
/// Generates a magnitude (dB) vs. frequency plot from 20 Hz to 20 kHz
/// with logarithmic frequency axis.
///
/// # Arguments
/// * `resp` - The frequency response to plot
/// * `path` - Output SVG file path
/// * `width` - Image width in pixels (default: 800)
/// * `height` - Image height in pixels (default: 600)
///
/// # Returns
/// `Ok(())` on success, or an error if plotting or file I/O fails.
pub fn plot_response_svg(
    resp: &Response,
    path: &Path,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create SVG backend
    let root = SVGBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // Sample the response from 20 Hz to 20 kHz
    let nyquist = resp.nyquist();
    let f_max = 20_000.0_f64.min(nyquist);
    let f_min = 20.0_f64;
    
    // Generate sample points (log-spaced)
    let n_points = 1000;
    let log_min = f_min.ln();
    let log_max = f_max.ln();
    let log_step = (log_max - log_min) / (n_points - 1) as f64;
    
    let mut points = Vec::with_capacity(n_points);
    let mut db_min = 0.0;
    let mut db_max = 0.0;
    
    for i in 0..n_points {
        let log_f = log_min + i as f64 * log_step;
        let f = log_f.exp();
        let db = resp.db_at(f);
        points.push((f, db));
        
        if i == 0 || db < db_min {
            db_min = db;
        }
        if i == 0 || db > db_max {
            db_max = db;
        }
    }
    
    // Add some margin to the dB range
    let db_margin = (db_max - db_min).max(10.0) * 0.1;
    db_min -= db_margin;
    db_max += db_margin;
    
    // Build chart with log frequency axis
    let mut chart = ChartBuilder::on(&root)
        .caption("Frequency Response", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (f_min..f_max).log_scale(),
            db_min..db_max,
        )?;
    
    chart.configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Magnitude (dB)")
        .draw()?;
    
    // Draw the response curve
    chart.draw_series(LineSeries::new(
        points.into_iter(),
        &BLUE,
    ))?;
    
    root.present()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freq_response;
    use std::env;
    
    #[test]
    fn plot_creates_svg_file() {
        let src = "out1 = in1;";
        let resp = freq_response(src, 48_000.0, 8192);
        
        let temp_dir = env::temp_dir();
        let svg_path = temp_dir.join("opengen_test_plot_module.svg");
        
        // Remove if exists
        std::fs::remove_file(&svg_path).ok();
        
        // Plot
        plot_response_svg(&resp, &svg_path, 800, 600)
            .expect("plot should succeed");
        
        // Verify file exists and contains SVG
        assert!(svg_path.exists(), "SVG file should exist");
        let content = std::fs::read_to_string(&svg_path)
            .expect("should read SVG");
        assert!(content.contains("<svg"), "should contain SVG tag");
        
        // Clean up
        std::fs::remove_file(&svg_path).ok();
    }
}
