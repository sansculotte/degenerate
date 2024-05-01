extern crate cairo;
extern crate clap;
extern crate hound;
extern crate image;

mod args;
mod feed;
mod ghostweb;
mod render;

use args::Args;
use args::Method;
use cairo::{Context, Format, ImageSurface};
use clap::Parser;
use degenerate::{load_soundfile, ramp, save_frame};
use ghostweb::{ghostweb, load_image};
use pbr::ProgressBar;
use std::convert::TryInto;

macro_rules! validate {
    ($e:expr, $msg:expr) => {
        if !$e {
            return Err(Error::BadRequest($msg.into()));
        }
    };
}

fn main() {
    let args = Args::parse();
    let radius = if args.radius > 0. {
        args.radius
    } else {
        (args.width / 2) as f64
    };
    multi_frame(radius, args)
}

fn multi_frame(radius: f64, args: Args) {
    let frames: usize;
    let duration: f64;
    let blocksize: usize;
    let samples: Vec<i32>;
    let mut radius = radius;
    let image = if args.image.is_empty() {
        None
    } else {
        load_image(&args.image, args.scale_image)
    };
    let (is, xs): (u32, Vec<ghostweb::Feed>) = match image {
        None => (0, vec![]),
        Some((is, xs)) => (is, xs),
    };

    let iterations = if args.iterations > 0 {
        args.iterations
    } else {
        match is {
            0 => args.width * args.height,
            _ => is,
        }
    };

    if args.soundfile.is_empty() {
        blocksize = 255;
        frames = if args.frames > 0 { args.frames } else { 1 };
        duration = frames as f64 / args.fps as f64;
        samples = ramp(blocksize * frames);
    } else {
        // the compiler doesn't like destructuring assignment
        let result = load_soundfile(args.soundfile.clone(), args.fps, args.frames, args.debug);
        blocksize = result.0;
        frames = result.1;
        duration = result.2;
        samples = result.3;
    }
    let mut block_iterator = samples.chunks(blocksize).skip(args.start);

    let basename = args.filename.clone();
    let outdir = args.outdir.clone();
    let mut pb = ProgressBar::new(frames as u64);
    let end = args.start + frames;

    for i in args.start..end {
        let block: Vec<i32>;
        let t = i as f64 / duration as f64 * args.t;
        let filename = format!("{}{}", basename, format!("{:01$}", i, 6));
        radius = radius * args.expansion;

        block = block_iterator
            .next()
            .unwrap()
            .try_into()
            .expect("could not unwrap soundfile sample block");

        let config =
            render::RenderConfig::new(iterations, args.method.clone(), radius, block, t, &args);
        let frame = match xs[..] {
            [] => render_frame(config, args.debug),
            _ => render_displacement_frame(config, &xs, i as f64 / frames as f64, args.debug),
        };
        save_frame(frame, &outdir, &filename);
        pb.inc();
    }
    pb.finish_print("done!");
}

fn render_frame(conf: render::RenderConfig, debug: bool) -> ImageSurface {
    let surface =
        ImageSurface::create(Format::ARgb32, conf.width as i32, conf.height as i32).unwrap();
    let context = Context::new(&surface).unwrap();
    let xs = ghostweb(
        conf.iterations,
        &conf.block,
        conf.radius,
        conf.f1,
        conf.f2,
        conf.m,
        conf.t,
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
    strength: f64,
) -> Vec<ghostweb::Feed> {
    pixels
        .into_iter()
        .zip(dx)
        .map(|(p, x)| ghostweb::Feed {
            p1: ghostweb::Point {
                x: p.p1.x * (1. - strength) + x.p1.x * strength,
                y: p.p1.y * (1. - strength) + x.p1.y * strength,
                z: p.p1.z * (1. - strength) + x.p1.z * strength,
            },
            p2: ghostweb::Point {
                x: p.p1.x * (1. - strength) + x.p2.x * strength,
                y: p.p1.y * (1. - strength) + x.p2.y * strength,
                z: p.p1.z * (1. - strength) + x.p2.z * strength,
            },
            radius: p.radius * (1. - strength) + x.radius * strength,
        })
        .collect()
}

fn render_displacement_frame(
    conf: render::RenderConfig,
    pixels: &Vec<ghostweb::Feed>,
    strength: f64,
    debug: bool,
) -> ImageSurface {
    let surface =
        ImageSurface::create(Format::ARgb32, conf.width as i32, conf.height as i32).unwrap();
    let context = Context::new(&surface).unwrap();
    let xs = ghostweb(
        conf.iterations,
        &conf.block,
        conf.radius,
        conf.f1,
        conf.f2,
        conf.m,
        conf.t,
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
                } else {
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
