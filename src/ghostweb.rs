use std::f64::consts::{E, PI, SQRT_2};

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
    block: Vec<f64>,
    radius: f64,
    m: f64
) -> Vec<Feed> {

    let mut r: f64 = radius;
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

    for i in 0..iterations {

        let index =  i as usize % block.len();
        let sample = block[index];

        c = (i as f64 / iterations as f64) * PI * 2.0;
        c2 = c * E;
        c3 = c * PHI;

        rf = c / 2. + 0.15;
        n = rf * m * (1. - m);

        if z2 > 0. {
            x1 = (c * z1).sin() * (c3 * r).cos();
        }
        else {
            x1 = (c * z2).sinh() * (c3 * r.sqrt()).cos();
        }
        y1 = -(c2 * z1).cos() + sample.powf(E);
        z1 = (x1 * sample as f64).cos();

        if z1 > 0. {
            x2 = c2.sin() * (c * r).cos();
        }
        else {
            x2 = (c3 * (r * n).sqrt()).atan() * (c * n - c2).cos();
        }
        y2 = c3.cos() - z1;
        z2 = ((x2 + y2) * sample as f64).cos();

        r = radius * n;
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
