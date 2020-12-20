extern crate image;
extern crate structopt;

mod ghostweb;

use ghostweb::ghostweb;
use std::path::PathBuf;
use structopt::StructOpt;


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
    outfile: String,

    #[structopt(short, long, default_value = "0.")]
    blur: f32

}

fn main() {

    let opt = Opt::from_args();

    let width = opt.width;
    let height = opt.height;
    let iterations =
        if opt.iterations > 0
              { opt.iterations }
        else  { opt.width * opt.height * 64 }; 

    // Create a new ImgBuf with width: width and height: height
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    // and set all to black
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([0]);
    }

    let zs = ghostweb(width, height, iterations, opt.radius, opt.m);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([(zs[x as usize][y as usize] * 255.) as u8]);
    }

    if opt.blur > 0. {
        imgbuf = image::imageops::blur(&imgbuf, opt.blur);
    }

    let outfile = PathBuf::from(opt.outfile);
    imgbuf.save(outfile).unwrap();
}
