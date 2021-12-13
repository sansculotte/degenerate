use crate::lib::{normalize, rms};
use noise::{Billow, HybridMulti, NoiseFn, OpenSimplex};
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

#[derive(Debug)]
struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
struct State {
    // current iteraton
    pub i: u32,

    pub sample: f64,

    pub c: f64,
    pub c2: f64,
    pub c3: f64,
    pub c4: f64,

    pub r: f64,

    pub p1: Point,
    pub p2: Point,

    // logistic map variables
    pub n: f64,
    pub rf: f64,

    // random state machines
    pub osx: OpenSimplex,
    pub hbm: HybridMulti,
    pub billow: Billow,
}

struct Parameter {
    iterations: u32,
    samples: Vec<f64>,
    radius: f64,
    m: f64,
    t: f64,
    rms: f64,
}

pub fn ghostweb(
    iterations: u32,
    block: &[i32],
    radius: f64,
    f1: usize,
    f2: usize,
    m: f64,
    t: f64
) -> Vec<Feed> {

    // collected points
    let mut xs: Vec<Feed> = vec![];

    let params = Parameter {
        iterations: iterations,
        samples: normalize(block),
        radius: radius,
        m: m,
        t: t,
        rms: rms(block),
    };
    let mut state = State {
        i: 0,
        sample: 0.,
        c: 0.,
        c2: 0.,
        c3: 0.,
        c4: 0.,
        p1: Point {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        p2: Point {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        n: 0.,
        rf: 0.,
        osx: OpenSimplex::new(),
        hbm: HybridMulti::new(),
        billow: Billow::new(),
        r: radius,
    };

    let equation_1 = select_equation(f1);
    let equation_2 = select_equation(f2);

    for i in 0..iterations {

        state = advance(i, state, &params);

        state.p1 = equation_1(&state, &params, &state.p1, &state.p2);
        state.p2 = equation_2(&state, &params, &state.p2, &state.p1);

        xs.push(Feed {
            x1: state.p1.x,
            y1: state.p1.y,
            z1: state.p1.z,
            x2: state.p2.x,
            y2: state.p2.y,
            z2: state.p2.z,
            radius: state.r,
        });
    }
    xs
}

fn advance(i: u32, mut state: State, p: &Parameter) -> State {
    let part = i as f64 / p.iterations as f64;

    if p.samples.len() > 0 {
        let index = i as usize % p.samples.len();
        state.sample = p.samples[index];
    } else {
        state.sample = 0.;
    }

    state.i = i;
    state.c =  part * PI * 2.0;
    state.c2 = state.c * E;
    state.c3 = state.c * PHI;
    state.c4 = state.c * SQRT_2;

    state.rf = state.c / 2. + 0.15;
    state.n = state.rf * p.m * (1. - p.m);
    state.r = p.radius * part;
    state
}

fn equation_000(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x: f64 = s.c.sin();
    let y: f64 = s.c.cos();
    let z: f64 = s.sample;
    Point { x: x, y: y, z: z }
}

fn equation_001(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x: f64 = s.c.sin();
    let y: f64 = (x.powf(3.) + 0.5 * x + 0.3333).sqrt();
    let z: f64 = p2.z;
    Point { x: x, y: y, z: z }
}

fn equation_002(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let r = s.c / (2. * PI);
    let x: f64 = s.c.cos() * r;
    let y: f64 = s.c.sin() * r;
    let z: f64 = ((x + y) * PI).tanh();
    Point { x: x, y: y, z: z }
}

fn equation_003(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x: f64 = (p.t + s.c * p2.z).sin()
        * (s.c2 * p.t.powf(s.c3)).cos()
        * s.hbm.get([p2.x, p2.y, p2.z]);
    let y: f64 = (p.t * E + s.c).sin() * (p.rms - p.t.powf(s.sample)).sin();
    let z: f64 = s.sample * s.osx.get([p1.x, p1.y, p.t]);
    Point { x: x, y: y, z: z }
}

fn equation_004(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x = ((s.c2 + p.t) + p1.z + s.n + p.rms).sin();
    let y = (s.c3 + p.t).cos() * s.billow.get([p1.x, p1.y, p.t * 2000.]);
    let z = (s.sample * p.rms + p.t).sin() * s.c;
    Point { x: x, y: y, z: z }
}

fn equation_005(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x = (s.c3 * s.r * s.n).sin() + (s.c * p.t - s.c2).cos() - (p2.z + E).ln().sin();
    let y = s.c3.cos() * (s.n * p1.z + p.rms).cos() + (p.t - (s.sample as f64).powf(E)).sin();
    let z = s.hbm.get([p1.x, p1.y, s.sample as f64])
        + s.billow.get([p2.x, p2.y, p2.z]) * s.sample;
    Point { x: x, y: y, z: z }
}

fn equation_006(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let mut x = s.c.sin() + (p2.z + p.t).ln() * (s.c2 * p2.z).cos() - s.sample;
    let mut y = (s.c.cos() + (s.c2 * p1.z).sin() + (x + p1.z).powf(s.c3) * s.r).atan() / ATAN_SATURATION;
    let mut z = (x.sin() + p.t * PI * (y + p2.z).cos() + s.c.powf(E) + (E * s.c.cos() * (SQRT_2 * y).cos()) + s.c3.sqrt() * s.c2.cos()).atan() / ATAN_SATURATION;
    if x.is_nan() { x = s.osx.get([p1.x, p1.y, p.t]) };
    if y.is_nan() { y = s.osx.get([p1.x, p.t, p1.z]) };
    if z.is_nan() { z = s.osx.get([p.t, p1.y, p1.z]) };
    Point { x: x, y: y, z: z }
}

fn equation_007(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let mut x = ((s.c * s.n * p.t).sin() + (s.c3 + p2.y).cos() * p1.z + s.sample * p2.z).atan() / ATAN_SATURATION;
    let mut y = (s.c.cos() + (s.c2 * p2.x).sin() * p1.x * p2.z + (p1.z.abs() + p2.z.abs()).sqrt()).atan() / ATAN_SATURATION;
    let mut z = (s.c2.sinh() + p.t.powf(s.c) + s.c3.tanh()).atan() / ATAN_SATURATION;
    if x.is_nan() { x = s.hbm.get([p.t, p1.y, p1.z]) };
    if y.is_nan() { y = s.hbm.get([p1.x, p.t, p1.z]) };
    if z.is_nan() { z = s.hbm.get([p1.x, p1.y, p.t]) };
    Point { x: x, y: y, z: z }
}

fn equation_008(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x = p1.x * s.sample + s.c2.sin() - p1.z * p.rms;
    let y = p2.y * (s.sample + 0.5) - s.c3.cos() * (s.c * s.n).sinh();
    let z = (x * s.c2 + p.t).sin() * (y * s.c * s.n).cos();
    Point { x: x, y: y, z: z }
}

fn equation_009(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let mut x = ((s.c * s.n + p2.x).cos() + (s.c3 + p2.z).sin() - p1.z * (s.c + s.n + p2.y).abs().sqrt() * s.c2).atan() / ATAN_SATURATION;
    let mut y = ((s.c2 * (x * PI + p2.x * s.sample).powf(PHI)).cos() + (s.c3.powf(p2.y)).sin() * p1.x * p2.z + (s.p1.z.abs() + p2.z.abs()).powf(p1.z)).atan() / ATAN_SATURATION;
    let mut z = (x.powf(s.c2) * y.powf(s.c3) - (p2.x + p1.x + p.rms).cos()).atan() / ATAN_SATURATION;
    if x.is_nan() { x = s.hbm.get([p1.x, p1.y, p.t]) };
    if y.is_nan() { y = s.hbm.get([p1.x, p.t, p1.z]) };
    if z.is_nan() { z = s.hbm.get([p.t, p1.y, p1.z]) };
    Point { x: x, y: y, z: z }
}

// totenschiff
fn equation_010(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let mut x = s.c.sin() * (s.sample / 4.) + s.c2.cos() * (s.sample / 5.) - (p1.z * s.n).cosh() * (s.sample / 7.) + (s.c4 * s.c3).cos() * (s.sample / 9.) - p1.y * ((s.c2 * s.c3).cos() * p1.y).sqrt();
    let mut y = s.c.cos() * (s.sample / 4.) + s.c2.sin() * (s.sample / 5.) - (p1.z * s.n).sinh() * (s.sample / 7.) + p1.y * (s.c2 * s.c3).sqrt() - (s.sample / 11.) * (s.c4 * s.c3).sin();
    let mut z =
        ((((x * s.r).sin() * PI * (y + p2.z)).cos() + (s.c + p1.z).powf(E) - PHI * s.c.cos() * (p1.z + y).powf(E) * (s.c3 * s.c2).ln() * s.c2.cos().powf(2.)).atan() / ATAN_SATURATION)
        * s.n;
    if x.is_nan() { x = s.osx.get([p.t, p1.y, p1.z]) };
    if y.is_nan() { y = s.osx.get([p1.x, p.t, p1.z]) };
    if z.is_nan() { z = s.osx.get([p1.x, p1.y, p.t]) };
    if !x.is_finite() { x = s.hbm.get([p.t, p1.y, p1.z]) };
    if !y.is_finite() { y = s.hbm.get([p1.x, p.t, p1.z]) };
    if !z.is_finite() { z = s.hbm.get([p1.x, p1.y, p.t]) };
    Point { x: x, y: y, z: z }
}

fn equation_011(s: &State, _p: &Parameter, _p1: &Point, _p2: &Point) -> Point {
    let x = s.c.sin() + (2. * s.c.powf(2.)).sin() * s.c.cos();
    let y = s.c.cos() + s.c.powf(2.).sin();
    let z = x * y;
    Point { x: x, y: y, z: z }
}

fn equation_012(s: &State, p: &Parameter, p1: &Point, p2: &Point) -> Point {
    let x = (s.c * p.t).cos() + p1.x / SQRT_2 - p2.x / E;
    let y = s.sample * 1. / (p1.z.abs() * s.c2 + p.t).ln() * (x * PI + p1.z * E + p2.z * SQRT_2).sin();
    let z = s.osx.get([s.sample, p.t, x]);
    Point { x: x, y: y, z: z }
}


fn select_equation(index: usize) -> fn(&State, &Parameter, p1: &Point, p2: &Point) -> Point {
    match index {
        1  => equation_001,
        2  => equation_002,
        3  => equation_003,
        4  => equation_004,
        5  => equation_005,
        6  => equation_006,
        7  => equation_007,
        8  => equation_008,
        9  => equation_009,
        10 => equation_010,
        11 => equation_011,
        12 => equation_012,
        _  => equation_000,
    }
}
