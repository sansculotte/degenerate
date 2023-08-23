extern crate cairo;
extern crate hound;
extern crate structopt;
extern crate image;

mod feed;
mod ghostweb;
mod lib;

use cairo::{Context, Format, ImageSurface};
use ghostweb::ghostweb;
use ghostweb::load_image;
use pbr::ProgressBar;
use std::convert::TryInto;
use structopt::StructOpt;
use lib::{load_soundfile, ramp, save_frame};


macro_rules! validate {
    ($e:expr, $msg:expr) => {
        if !$e {
            return Err(Error::BadRequest($msg.into()));
        }
    };
}

#[derive(Debug, Clone)]
enum Method {
    Arc,
    Curve,
    Dot,
    Line,
}

#[derive(Debug)]
struct RenderConfig {
        // iterations (point pairs) per frame
    iterations: u32,
    // expansion radius
    radius: f64,
    // time
    t: f64,
    // m parameter for exponential transfer function
    m: f64,
    f1: usize,
    f2: usize,
    block: Vec<i32>,
    width: u32,
    height: u32,
    method: Method,
    size: f64,
    combine_dots: bool,
}

impl RenderConfig {
    pub fn new(iterations: u32, method: Method, radius: f64, block: Vec<i32>, t: f64, opt: &Opt) -> Self {
        Self {
            iterations,
            radius,
            t,
            m: opt.m,
            f1: opt.f1,
            f2: opt.f2,
            block,
            width: opt.width,
            height: opt.height,
            method,
            size: opt.size,
            combine_dots: opt.combine_dots,
        }
    }
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "degenerate",
    about = "Generative and manipulative Images from arithmetic primitives",
    version = env!("CARGO_PKG_VERSION")
)]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, default_value = "4000")]
    width: u32,

    #[structopt(short, long, default_value = "4000")]
    height: u32,

    #[structopt(short, long, default_value = "0")]
    iterations: u32,

    #[structopt(long, default_value = "25")]
    fps: usize,

    #[structopt(long, default_value = "0")]
    f1: usize,

    #[structopt(long, default_value = "0")]
    f2: usize,

    #[structopt(short = "t", default_value = "1.0")]
    t: f64,

    #[structopt(short, long, default_value = "0")]
    radius: f64,

    #[structopt(short, long, default_value = "1.0")]
    expansion: f64,

    #[structopt(long)]
    combine_dots: bool,

    #[structopt(short = "M", long, parse(try_from_str = parse_method), default_value = "dot")]
    method: Method,

    #[structopt(long, default_value = "1")]
    scale_image: f64,

    #[structopt(short, long, default_value = "0")]
    size: f64,

    #[structopt(short, default_value = "0.2")]
    m: f64,

    #[structopt(short, long, default_value = "/tmp")]
    outdir: String,

    #[structopt(short = "n", long, default_value = "frame_")]
    filename: String,

    #[structopt(long, default_value = "0")]
    start: usize,

    #[structopt(short, long, default_value = "0")]
    frames: usize,

    #[structopt(long, default_value = "")]
    image: String,

    #[structopt(default_value = "")]
    soundfile: String,
}

fn main() {
    let opt = Opt::from_args();

    let radius = if opt.radius > 0. {
        opt.radius
    } else {
        (opt.width / 2) as f64
    };

    multi_frame(radius, opt)
}

fn multi_frame(radius: f64, opt: Opt) {
    let frames: usize;
    let duration: f64;
    let blocksize: usize;
    let samples: Vec<i32>;
    let mut radius = radius;
    let image = if opt.image.is_empty() {
        None
    } else {
        load_image(&opt.image, opt.scale_image)
    };
    let (is, xs): (u32, Vec<ghostweb::Feed>) = match image {
        None => (0, vec!()),
        Some((is, xs)) => (is, xs)
    };

    let iterations = if opt.iterations > 0 {
        opt.iterations
    } else {
        match is {
            0 => opt.width * opt.height,
            _ => is
        }
    };

    if opt.soundfile.is_empty() {
        blocksize = 255;
        frames = if opt.frames > 0 { opt.frames } else { 1 };
        duration = frames as f64 / opt.fps as f64;
        samples = ramp(blocksize * frames);
    } else {
        // the compiler doesn't like destructuring assignment
        let result = load_soundfile(
            opt.soundfile.clone(),
            opt.fps,
            opt.frames,
            opt.debug
        );
        blocksize = result.0; 
        frames = result.1;
        duration = result.2;
        samples = result.3;
    }
    let mut block_iterator = samples.chunks(blocksize).skip(opt.start);

    let basename = opt.filename.clone();
    let outdir = opt.outdir.clone();
    let mut pb = ProgressBar::new(frames as u64);
    let end = opt.start + frames;

    for i in opt.start..end {

        let block: Vec<i32>;
        let t = i as f64 / duration as f64 * opt.t;
        let filename = format!("{}{}", basename, format!("{:01$}", i, 6));
        radius = radius * opt.expansion;

        block = block_iterator
            .next()
            .unwrap()
            .try_into()
            .expect("could not unwrap soundfile sample block");

        let config = RenderConfig::new(iterations, opt.method.clone(), radius, block, t, &opt);
        let frame = match xs[..] {
            [] => {
                render_frame(config, opt.debug)
            },
            _ => {
                render_displacement_frame(config, &xs, i as f64 / frames as f64, opt.debug)
            }
        };
        save_frame(
            frame,
            &outdir,
            &filename
        );
        pb.inc();
    }
    pb.finish_print("done!");
}

fn render_frame(conf: RenderConfig, debug: bool) -> ImageSurface {
    let surface = ImageSurface::create(
        Format::ARgb32,
        conf.width as i32,
        conf.height as i32
    ).unwrap();
    let context = Context::new(&surface).unwrap();
    let xs = ghostweb(
        conf.iterations,
        &conf.block,
        conf.radius,
        conf.f1,
        conf.f2,
        conf.m,
        conf.t
    );
    draw_frame(
        &context,
        &xs,
        conf.width,
        conf.height,
        conf.size,
        debug,
        &conf.method,
        conf.combine_dots,
    );
    surface
}

fn displace(
    pixels: &Vec<ghostweb::Feed>,
    dx: &Vec<ghostweb::Feed>,
    strength: f64
) -> Vec<ghostweb::Feed> {
    pixels.into_iter().zip(dx).map(|(p, x)| {
        ghostweb::Feed {
            p1: ghostweb::Point {
                x: p.p1.x * (1.-strength) + x.p1.x * strength,
                y: p.p1.y * (1.-strength) + x.p1.y * strength,
                z: p.p1.z * (1.-strength) + x.p1.z * strength,
            },
            p2: ghostweb::Point {
                x: p.p1.x * (1.-strength) + x.p2.x * strength,
                y: p.p1.y * (1.-strength) + x.p2.y * strength,
                z: p.p1.z * (1.-strength) + x.p2.z * strength,
            },
            radius: p.radius * (1.-strength) + x.radius * strength
        }
    }).collect()
}

fn render_displacement_frame(
    conf: RenderConfig,
    pixels: &Vec<ghostweb::Feed>,
    strength: f64,
    debug: bool
) -> ImageSurface {
    let surface = ImageSurface::create(
        Format::ARgb32,
        conf.width as i32,
        conf.height as i32
    ).unwrap();
    let context = Context::new(&surface).unwrap();
    let xs = ghostweb(
        conf.iterations,
        &conf.block,
        conf.radius,
        conf.f1,
        conf.f2,
        conf.m,
        conf.t
    );
    draw_frame(
        &context,
        &displace(&pixels, &xs, strength),
        conf.width,
        conf.height,
        conf.size,
        debug,
        &conf.method,
        conf.combine_dots,
    );
    surface
}

fn draw_frame(
    context: &Context,
    xs: &Vec<ghostweb::Feed>,
    width: u32,
    height: u32,
    size: f64,
    debug: bool,
    method: &Method,
    combine_dots: bool,
) {
    let cx: f64 = width as f64 / 2.;
    let cy: f64 = height as f64 / 2.;

    // black out
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint().unwrap();

    for x in xs {
        if debug {
            println!("{:?}", x);
        }

        let crx1 = cx + x.p1.x * x.radius;
        let cry1 = cy + x.p1.y * x.radius;
        let crx2 = cx + x.p2.x * x.radius;
        let cry2 = cy + x.p2.y * x.radius;
        let crx3 = cx + cx * x.p1.x + x.p2.x * x.radius;
        let cry3 = cy + cy * x.p1.y + x.p2.y * x.radius;

        context.set_line_width(0.1);
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.move_to(crx1, cry1);

        match method {
            Method::Arc => context.arc(crx1, cry1, x.radius, x.p1.z, x.p2.z),
            Method::Curve => context.curve_to(
                crx1,
                cry1,
                crx2,
                cry2,
                cx + x.p1.z * x.radius,
                cy + x.p2.z * x.radius,
            ),
            Method::Dot => {
                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                if combine_dots {
                    context.rectangle(crx3, cry3, 0.5, 0.5);
                }
                else {
                    let size_1 = if size > 0. { x.p1.z.abs() * size } else { 1.0 };
                    let size_2 = if size > 0. { x.p2.z.abs() * size } else { 1.0 };
                    context.rectangle(crx1, cry1, size_1, size_1);
                    context.stroke().unwrap();
                    context.fill().unwrap();
                    context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                    context.rectangle(crx2, cry2, size_2, size_2);
                }
            }
            Method::Line => context.line_to(crx2, cry2),
        }
        context.stroke().unwrap();
    }
}
