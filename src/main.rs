extern crate cairo;
extern crate structopt;

mod ghostweb;

use std::fs::File;
use structopt::StructOpt;
use cairo::{ ImageSurface, Format, Context };
use ghostweb::ghostweb;


#[derive(Debug)]
enum Method {
    Arc,
    Curve,
    Dot,
    Line
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
#[structopt(name = "degenerate", about = "Generative Images from mathematic primitives")]
struct Opt {

    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, default_value = "4000")]
    width: u32,

    #[structopt(short, long, default_value = "4000")]
    height: u32,

    #[structopt(short, long, default_value = "0")]
    iterations: u32,

    #[structopt(short, long, default_value = "0")]
    radius: f64,

    #[structopt(short = "M", long, parse(try_from_str = parse_method), default_value = "dot")]
    method: Method,

    #[structopt(short = "m", default_value = "0.2")]
    m: f64,

    #[structopt(short, long, default_value = "image.png")]
    outfile: String
}

fn main() {

    let opt = Opt::from_args();

    let width = opt.width;
    let height = opt.height;
    let cx: f64 = width as f64 / 2.;
    let cy: f64 = height as f64 / 2.;
    let iterations =
        if opt.iterations > 0
              { opt.iterations }
        else  { opt.width * opt.height };
    let radius =
        if opt.radius > 0.
              { opt.radius }
        else  { opt.width as f64 };

    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface);

    // black out
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint();

    let xs = ghostweb(iterations, radius, opt.m);

    for x in xs {
        if opt.debug {
            println!("{:?}", x);
        }

        let crx1 = cx + x.x1 * x.radius;
        let cry1 = cy + x.y1 * x.radius;
        let crx2 = cx + x.x2 * x.radius;
        let cry2 = cy + x.y2 * x.radius;

        context.set_line_width(0.1);
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.move_to(crx1, cry1);

        match opt.method {
            Method::Arc => context.arc(crx1, cry1, x.radius, x.z1, x.z2),
            Method::Curve => context.curve_to(crx1, cry1, crx2, cry2, cx + x.z1 * x.radius, cy + x.z2 * x.radius),
            Method::Dot => {
                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                context.rectangle(crx1, cry1, 0.5, 0.5);
                context.stroke();
                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                context.rectangle(crx2, cry2, 0.5, 0.5);
            },
            Method::Line => context.line_to(crx2, cry2),
        }

        context.stroke();
    }

    let mut outfile = File::create(opt.outfile)
        .expect("Could not open output file");

    surface.write_to_png(&mut outfile)
        .expect("Could not write to output file");
}
