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

        c = (i as f64 / iterations as f64) * PI * 2.0;
        c2 = c * E;
        c3 = c * PHI;

        rf = c / 2. + 0.15;
        n = rf * m * (1. - m);

        // these epicircles produce a very symmetric web
        // use n * or any other near chaotic function to break uo symmetry
        x1 = (c * z1).sin() * (c3 * r).cos();
        y1 = (c2 * z1).cos();
        z1 = (x1 * 2. * PI).cos();
        /*
        z1 = (
            //(x as f64).sin() * (i as f64) + (y as f64).cos() * 2.3f64.powf(x as f64)
            (((x1 * r).sin() * PI * y1 + z1).cos() + c.ln() + E * c.cos() * SQRT_2 * y1.cos() * c3.sqrt() * c2.cos())
            *
            ((((x1 * c).sin() * PI * y1 + z1)).cos() + (c * z1).powf(E) + PHI * z2.cos() * (z1 + y1 * c3).powf(E) * (c3 * c2).ln() * c2.cos().powf(n))
        ).atan() / ATAN_SATURATION;
        if z1.is_nan() {
            z1 = 1.0;
        }
        */

        x2 = c2.sin() * (c * r).cos();
        y2 = (c3 * n.ln()).cos();
        z2 = (
            c.cos() * c.tan() * c3.cos() * (x2 * c + z2.powf(c)).sin()
            * n
            * c.powf(SQRT_2).cos() * c2.powf(3.).tan() * c3.powf(2.).cos() * (x2 * r  + z2.powf(c)).sin()
        ).atan() / ATAN_SATURATION;
        if z2.is_nan() {
            z2 = 1.0;
        }

        r -= radius / iterations as f64;
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
