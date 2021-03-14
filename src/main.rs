extern crate cairo;
extern crate structopt;

mod ghostweb;

use ghostweb::ghostweb;
use std::fs::File;
use structopt::StructOpt;
use cairo::{ ImageSurface, Format, Context };


#[derive(Debug, StructOpt)]
#[structopt(name = "degenerate", about = "Generative Images from mathematic primitives ")]
struct Opt {

    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, default_value = "4000")]
    width: u32,

    #[structopt(short, long, default_value = "4000")]
    height: u32,

    #[structopt(short, long, default_value = "0")]
    iterations: u32,

    #[structopt(short, long, default_value = "235.0")]
    radius: f64,

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
        else  { opt.width * opt.height * 64 }; 

    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface);

    // black out
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint();

    let xs = ghostweb(width, height, iterations, opt.radius, opt.m);

    for (x1, y1, x2, y2, length, strength) in xs {
        if opt.debug {
            println!("{} {} {} {} {} {}", x1, y1, x2, y2, length, strength);
        }
        context.set_line_width(0.1);
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.move_to(cx + x1 * cx, cy + y1 * cy);
        context.line_to(cx + x2 * cx, cy + y2 * cy);
        context.stroke();
    }

    let mut outfile = File::create(opt.outfile)
        .expect("Could not open output file");
    surface.write_to_png(&mut outfile)
        .expect("Could not write to output file");
}
