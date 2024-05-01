use cairo::ImageSurface;
use rustfft::{num_complex::Complex, FftPlanner};
use std::cmp;
use std::fs::File;
use std::path::Path;

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

    let mut buffer = samples
        .iter()
        .map(|x| Complex {
            re: *x as f32,
            im: 0.0f32,
        })
        .collect::<Vec<Complex<f32>>>();

    fft.process(&mut buffer);

    // scale
    buffer
        .iter()
        .map(|x| (x / size as f32).sqrt())
        .collect::<Vec<Complex<f32>>>()
}

pub fn rms_16(samples: &[i16]) -> f64 {
    let squared = samples
        .iter()
        // i16 because the soundfile is i16. needs to be passed from soundfile info!
        .map(|s| *s as f64 / i16::MAX as f64)
        .fold(0.0, |a, s| a + s * s);
    let mean = squared / samples.len() as f64;
    mean.sqrt()
}

pub fn rms_32(samples: &[i32]) -> f64 {
    let squared = samples
        .iter()
        .map(|s| *s as f64 / i32::MAX as f64)
        .fold(0.0, |a, s| a + s * s);
    let mean = squared / samples.len() as f64;
    mean.sqrt()
}

pub fn ramp(size: usize) -> Vec<i32> {
    (0..=size as i32).collect::<Vec<_>>()
}

pub fn load_soundfile(
    filename: String,
    fps: usize,
    frames: usize,
    debug: bool,
) -> (usize, usize, f64, Vec<i32>) {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let spec: hound::WavSpec = reader.spec();
    let duration = reader.duration() as f64;
    let blocksize: usize = (spec.sample_rate as usize / fps) * spec.channels as usize;
    let samples: Vec<i32> = reader.samples().map(|s| s.unwrap()).collect();
    let number_of_frames = if frames > 0 {
        cmp::min(frames, samples.len() / blocksize)
    } else {
        samples.len() / blocksize
    };

    if debug {
        println!("blocksize: {:?}", blocksize);
        println!("frames: {:?}", number_of_frames);
        println!("samples: {:?}", samples.len());
    }

    (blocksize, number_of_frames, duration, samples)
}

pub fn save_frame(surface: ImageSurface, outdir: &String, filename: &String) {
    let path = Path::new(outdir).join(format!("{}.png", filename));
    let mut outfile = File::create(path).expect("Could not open output file");
    surface
        .write_to_png(&mut outfile)
        .expect("Could not write to output file");
}
