extern crate cairo;
extern crate hound;
extern crate structopt;

mod ghostweb;
mod lib;

use cairo::{Context, Format, ImageSurface};
use ghostweb::ghostweb;
use pbr::ProgressBar;
use std::convert::TryInto;
use std::fs::File;
use std::path::Path;
use structopt::StructOpt;

const VERSION: &str = "0.0.2";

#[derive(Debug)]
enum Method {
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "degenerate",
    about = "Generative Images from mathematic primitives",
    version = VERSION
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

    #[structopt(long, default_value = "1")]
    f2: usize,

    #[structopt(short = "t", default_value = "1.0")]
    t: f64,

    #[structopt(short, long, default_value = "0")]
    radius: f64,

    #[structopt(short = "M", long, parse(try_from_str = parse_method), default_value = "dot")]
    method: Method,

    #[structopt(short = "m", default_value = "0.2")]
    m: f64,

    #[structopt(short, long, default_value = "/tmp")]
    outdir: String,

    #[structopt(default_value = "")]
    soundfile: String,
}

fn main() {
    let opt = Opt::from_args();

    let iterations = if opt.iterations > 0 {
        opt.iterations
    } else {
        opt.width * opt.height
    };
    let radius = if opt.radius > 0. {
        opt.radius
    } else {
        (opt.width / 2) as f64
    };

    if opt.soundfile == "" {
        single_frame(iterations, radius, opt)
    } else {
        multi_frame(iterations, radius, opt)
    }
}

fn multi_frame(iterations: u32, radius: f64, opt: Opt) {
    let width = opt.width;
    let height = opt.height;
    let method = opt.method;

    // load soundfile
    let mut reader = hound::WavReader::open(opt.soundfile).unwrap();
    let spec: hound::WavSpec = reader.spec();
    let duration = reader.duration();
    let blocksize: usize = (spec.sample_rate as usize / opt.fps) * spec.channels as usize;
    let samples: Vec<i32> = reader.samples().map(|s| s.unwrap()).collect();
    let frames = samples.len() / blocksize;

    // set up drawing canvas
    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface);

    let mut pb = ProgressBar::new(frames as u64);

    for (i, block) in samples.chunks(blocksize).enumerate() {
        let t = i as f64 / duration as f64 * opt.t;
        let xs = ghostweb(iterations, block, radius, opt.f1, opt.f2, opt.m, t);
        draw(&context, &xs, opt.width, opt.height, opt.debug, &method);

        let path = Path::new(&opt.outdir).join(format!("{:01$}.png", i, 6));

        let mut outfile = File::create(path).expect("Could not open output file");

        surface
            .write_to_png(&mut outfile)
            .expect("Could not write to output file");

        pb.inc();
    }
    pb.finish_print("done!");
}

fn single_frame(iterations: u32, radius: f64, opt: Opt) {
    let width = opt.width;
    let height = opt.height;
    let method = opt.method;

    // set up drawing canvas
    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface);
    let block: [i32; 256] = (0..=255).collect::<Vec<_>>().try_into().expect("wrong size iterator");

    let xs = ghostweb(iterations, &block, radius, opt.f1, opt.f2, opt.m, opt.t);
    draw(&context, &xs, opt.width, opt.height, opt.debug, &method);

    let path = Path::new(&opt.outdir).join(format!("image.png"));

    let mut outfile = File::create(path).expect("Could not open output file");

    surface
        .write_to_png(&mut outfile)
        .expect("Could not write to output file");
}

fn draw(
    context: &Context,
    xs: &Vec<ghostweb::Feed>,
    width: u32,
    height: u32,
    debug: bool,
    method: &Method,
) {
    let cx: f64 = width as f64 / 2.;
    let cy: f64 = height as f64 / 2.;

    // black out
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint();

    for x in xs {
        if debug {
            println!("{:?}", x);
        }

        let crx1 = cx + x.x1 * x.radius;
        let cry1 = cy + x.y1 * x.radius;
        let crx2 = cx + x.x2 * x.radius;
        let cry2 = cy + x.y2 * x.radius;

        context.set_line_width(0.1);
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.move_to(crx1, cry1);

        match method {
            Method::Arc => context.arc(crx1, cry1, x.radius, x.z1, x.z2),
            Method::Curve => context.curve_to(
                crx1,
                cry1,
                crx2,
                cry2,
                cx + x.z1 * x.radius,
                cy + x.z2 * x.radius,
            ),
            Method::Dot => {
                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                context.rectangle(crx1, cry1, 0.5, 0.5);
                context.stroke();
                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                context.rectangle(crx2, cry2, 0.5, 0.5);
            }
            Method::Line => context.line_to(crx2, cry2),
        }
        context.stroke();
    }
}
