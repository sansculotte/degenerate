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
    let iterations =
        if opt.iterations > 0
              { opt.iterations }
        else  { opt.width * opt.height * 64 }; 

    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface);

    // black out
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint();

    let zs = ghostweb(width, height, iterations, opt.radius, opt.m);

    for y in 0..height {
        for x in 0..width {
            let z = zs[x as usize][y as usize];
            context.set_source_rgba(1.0, 1.0, 1.0, z);
            context.rectangle(x as f64, y as f64, 0.1, 0.1);
            context.stroke();
        }
    }

    let mut outfile = File::create(opt.outfile)
        .expect("Could not open output file");
    surface.write_to_png(&mut outfile)
        .expect("Could not write to output file");
}
