use clap::Parser;

#[derive(Debug, Clone)]
pub enum Method {
    Arc,
    Curve,
    Dot,
    Line,
}

// to select a method by string for structopt
fn parse_method(method: &str) -> Result<Method, String> {
    match method {
        "arc" => Ok(Method::Arc),
        "curve" => Ok(Method::Curve),
        "dot" => Ok(Method::Dot),
        "line" => Ok(Method::Line),
        _ => Err(format!("Could not parse method {}", method)),
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "degenerate",
    about = "Generative and manipulative Images from arithmetic primitives and soundwaves",
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Args {
    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long, default_value = "4000")]
    pub width: u32,

    #[arg(short, long, default_value = "4000")]
    pub height: u32,

    #[arg(short, long, default_value = "0")]
    pub iterations: u32,

    #[arg(long, default_value = "25")]
    pub fps: usize,

    #[arg(long, default_value = "0")]
    pub f1: usize,

    #[arg(long, default_value = "0")]
    pub f2: usize,

    #[arg(short, default_value = "1.0")]
    pub t: f64,

    #[arg(short, long, default_value = "0")]
    pub radius: f64,

    #[arg(short, long, default_value = "1.0")]
    pub expansion: f64,

    #[arg(long)]
    pub combine_dots: bool,

    #[arg(short = 'M', long, value_parser = parse_method, default_value = "dot")]
    pub method: Method,

    #[arg(long, default_value = "1")]
    pub scale_image: f64,

    #[arg(short, long, default_value = "0")]
    pub size: f64,

    #[arg(short, default_value = "0.2")]
    pub m: f64,

    #[arg(short, long, default_value = "/tmp")]
    pub outdir: String,

    #[arg(long, default_value = "frame_")]
    pub filename: String,

    #[arg(long, default_value = "0")]
    pub start: usize,

    #[arg(short, long, default_value = "0")]
    pub frames: usize,

    #[arg(long, default_value = "")]
    pub image: String,

    #[arg(default_value = "")]
    pub soundfile: String,
}
