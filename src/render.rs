use super::args::{Args, Method};

#[derive(Debug)]
pub struct RenderConfig {
    // iterations (point pairs) per frame
    pub iterations: u32,
    // expansion radius
    pub radius: f64,
    // time
    pub t: f64,
    // m parameter for exponential transfer function
    pub m: f64,
    pub f1: usize,
    pub f2: usize,
    pub block: Vec<i32>,
    pub width: u32,
    pub height: u32,
    pub method: Method,
    pub size: f64,
    pub combine_dots: bool,
}

impl RenderConfig {
    pub fn new(
        iterations: u32,
        method: Method,
        radius: f64,
        block: Vec<i32>,
        t: f64,
        args: &Args,
    ) -> Self {
        Self {
            iterations,
            radius,
            t,
            m: args.m,
            f1: args.f1,
            f2: args.f2,
            block,
            width: args.width,
            height: args.height,
            method,
            size: args.size,
            combine_dots: args.combine_dots,
        }
    }
}
