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

struct Parameter<'a> {
    iterations: u32,
    block: &'a [i32],
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
    let samples = normalize(block);

    // collected points
    let mut xs: Vec<Feed> = vec![];

    let params = Parameter {
        iterations: iterations,
        block: block,
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
        if block.len() > 0 {
            let index = i as usize % block.len();
            state.sample = samples[index];
        } else {
            state.sample = 0.;
        }

        state = advance(state, &params);

        state.i = i;
        state.r = radius; // * state.n; //(state.n + params.rms);
        state.p1 = equation_1(&state, &params);
        state.p2 = equation_2(&state, &params);

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

fn advance(mut state: State, p: &Parameter) -> State {
    state.c = (state.i as f64 / p.iterations as f64) * PI * 2.0;
    state.c2 = state.c * E;
    state.c3 = state.c * PHI;

    state.rf = state.c / 2. + 0.15;
    state.n = state.rf * p.m * (1. - p.m);
    state
}

fn equation_000(s: &State, p: &Parameter) -> Point {
    let x: f64 = s.c.sin();
    let y: f64 = s.c.cos();
    let z: f64 = s.p1.z;
    Point { x: x, y: y, z: z }
}

fn equation_001(s: &State, p: &Parameter) -> Point {
    let x: f64 = s.c.sin();
    let y: f64 = (x.powf(3.) + 0.5 * x + 0.3333).sqrt();
    let z: f64 = s.p1.z;
    Point { x: x, y: y, z: z }
}

fn equation_002(s: &State, p: &Parameter) -> Point {
    let r = s.c / (2. * PI);
    let x: f64 = s.c.cos() * r;
    let y: f64 = s.c.sin() * r;
    let z: f64 = ((x + y) * PI).tanh();
    Point { x: x, y: y, z: z }
}

fn equation_003(s: &State, p: &Parameter) -> Point {
    let x: f64 = (p.t + s.c * s.p2.z).sin()
        * (s.c2 * p.t.powf(s.c3)).cos()
        * s.hbm.get([s.p2.x, s.p2.y, s.p2.z]);
    let y: f64 = (p.t * 4000. + s.c).sin() * (p.rms - p.t.powf(s.sample as f64)).sin();
    let z: f64 = s.sample * s.osx.get([s.p1.x, s.p1.y, p.t]);
    Point { x: x, y: y, z: z }
}

fn equation_004(s: &State, p: &Parameter) -> Point {
    let x = ((s.c2 + p.t) + s.p1.z + s.n).sin();
    let y = (s.c3 + p.t).cos() * s.billow.get([s.p1.x, s.p1.x, p.t * 2000.]);
    let z = (s.sample * p.rms + p.t) * s.c;
    Point { x: x, y: y, z: z }
}

fn equation_005(s: &State, p: &Parameter) -> Point {
    let x = (s.c3 * s.r * s.n).sin() + (s.c * p.t - s.c2).cos();
    let y = s.c3.cos() * (s.n * p.t + p.rms).cos() + (p.t + (s.sample as f64).powf(E)).sin();
    let z = s.hbm.get([s.p1.x, s.p1.y, s.sample as f64])
        + s.billow.get([s.p2.x, s.p2.y, s.p2.z]) * s.sample;
    Point { x: x, y: y, z: z }
}

fn select_equation(index: usize) -> fn(&State, &Parameter) -> Point {
    match index {
        1 => equation_001,
        2 => equation_002,
        3 => equation_003,
        4 => equation_004,
        5 => equation_005,
        _ => equation_000,
    }
}
