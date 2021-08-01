use noise::{NoiseFn, Billow, OpenSimplex, HybridMulti};
use std::f64::consts::{E, PI, SQRT_2};
use crate::lib::{normalize, rms};

const PHI: f64 = 1.618033988749;
const ATAN_SATURATION: f64 = 1.569796;


#[derive(Debug)]
pub struct Feed {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
    pub radius: f64,
}


pub fn ghostweb(
    iterations: u32,
    block: &[i32],
    radius: f64,
    m: f64,
    t: f64,
) -> Vec<Feed> {

    let rms = rms(block);
    let samples = normalize(block);

    let mut c: f64;
    let mut c2: f64;
    let mut c3: f64;
    let mut x1: f64;
    let mut y1: f64;
    let mut x2: f64;
    let mut y2: f64;
    let mut z1: f64 = 1.;
    let mut z2: f64 = 1.;

    // logistic map variables
    let mut n: f64;
    let mut rf: f64;
    let mut xs: Vec<Feed> = Vec::new();

    let osn = OpenSimplex::new();
    let hbm = HybridMulti::new();
    let billow = Billow::new();

    for i in 0..iterations {

        let sample: f64;
        if block.len() > 0 {
            let index = i as usize % block.len();
            sample = samples[index];
        }
        else {
            sample = 0.;
        }

        c = (i as f64 / iterations as f64) * PI * 2.0;
        c2 = c * E;
        c3 = c * PHI;

        rf = c / 2. + 0.15;
        n = rf * m * (1. - m);

        x1 = (t + c * z2).sin() * (c2 * t.powf(c3)).cos();
        y1 = (t * 4000. + c).sin() * (rms - t.powf(sample as f64)).sin();
        z1 = sample * osn.get([x1, y1, t]);

        x2 = ((c2 + t) + z1 + n).sin();
        y2 = (c3 + t).cos() * billow.get([x1, x2, t * 2000.]);
        z2 = (sample * rms + t) * c;

//        x2 = (c3 * r * n).sin() + (c * t - c2).cos();
//        y2 = c3.cos() * (n * t + rms).cos() + (t + (sample as f64).powf(E)).sin();
//        z2 = hbm.get([x1, y1, sample as f64]) + billow.get([x2, y2, z1]) * sample;

        let r = radius * (n + rms);
        xs.push(
            Feed {
                x1: x1,
                y1: y1,
                z1: z1,
                x2: x2,
                y2: y2,
                z2: z2,
                radius: r
            }
        );
    }
    xs
}
