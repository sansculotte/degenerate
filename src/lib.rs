use rustfft::{FftPlanner, num_complex::Complex};

fn min_max(samples: &[i32]) -> (i32, i32) {
    let min = *samples.iter().min().unwrap_or(&0);
    let max = *samples.iter().max().unwrap_or(&0);
    (min, max)
}

/*
 * Normalize sample slice to f64 -1..1
 */
pub fn normalize(samples: &[i32]) -> Vec<f64> {
    let (min, max) = min_max(&samples);
    if max - min > 0 {
        samples
            .iter()
            .map(|s| *s as f64 / (max - min).abs() as f64)
            .collect::<Vec<f64>>()
    } else {
        samples.iter().map(|s| *s as f64).collect::<Vec<f64>>()
    }
}

pub fn fft(samples: Vec<f64>) -> Vec<Complex<f32>> {
    let size = samples.len();
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(size);
    let mut buffer = vec![Complex{ re: 0.0, im: 0.0 }; size];
    fft.process(&mut buffer);
    buffer
}

pub fn rms(samples: &[i32]) -> f64 {
    // i16 beccause the soundfile is i16. needs to be passed from soundfile info!
    let squared = samples
        .iter()
        .map(|s| *s as f64 / i16::MAX as f64)
        .fold(0.0, |a, s| a + s * s);
    let mean = squared / samples.len() as f64;
    mean.sqrt()
}
