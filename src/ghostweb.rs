use noise::{NoiseFn, OpenSimplex, HybridMulti};
use std::f64::consts::{E, PI, SQRT_2};

const PHI: f64 = 1.618033988749;
const ATAN_SATURATION: f64 = 1.569796;


pub fn ghostweb(
    width: u32,
    height: u32,
    iterations: u32,
    radius: f64,
    m: f64
) -> std::vec::Vec<std::vec::Vec<f64>> {

    let cx: f64 = width as f64 / 2.;
    let cy: f64 = height as f64 / 2.;
    let mut r: f64 = radius;

    let mut c: f64;
    let mut c2: f64;
    let mut c3: f64;
    let mut x: u32;
    let mut y: u32;
    let mut z: f64 = 1.;
    let mut zs = vec![vec![0f64; height as usize]; width as usize];

    // logistic map variables
    let mut n: f64;
    let mut rf: f64;

    let osn = OpenSimplex::new();
    let hbm = HybridMulti::new();

    for i in 0..iterations {

        c = (i as f64 / iterations as f64) * PI * 2.0;
        c2 = c * E;
        c3 = c * PHI;

        rf = c / 2. + 0.15;
        n = rf * m * (1. - m);

        x = (cx + c.sin() * r + c.cos() * z * (height as f64)).round() as u32 % width;
        y = (cy + c.cos() * r + (c2 * z).sin() * (x as f64 * z * (width as f64) + E.powf(c2 / c3)).sqrt()).round() as u32 % height;

        z = osn.get([x as f64, y as f64]) + hbm.get([x as f64, y as f64]) * n;

        /*
        z = (
                //(x as f64).sin() * (i as f64) + (y as f64).cos() * 2.3f64.powf(x as f64)
                ((((x as f64).sin() * PI * (y as f64 + z)).cos() + c.ln() + (E * c.cos()) * (SQRT_2 * (y as f64).cos()) * c3.sqrt() * c2.cos()).atan() / ATAN_SATURATION)
                *
                ((((x as f64).sin() * PI * (y as f64 + z)).cos() + (c + z).powf(E) + PHI * c.cos() * (z + y as f64).powf(E) * (c3 * c2).ln() * c2.cos().powf(2.)).atan() / ATAN_SATURATION)
                //c.cos() * c.tan() * c3.cos() * (x as f64 + z.powf(c)).sin()
                * (c.powf(SQRT_2).cos() * c2.powf(3.).tan() * c3.powf(2.).cos() * (x as f64 + z.powf(c)).sin()).atan() / ATAN_SATURATION
        ).abs();
        */
        r -= 235.0 / iterations as f64;
        zs[x as usize][y as usize] += z.powi(2);
    }
    zs
}
