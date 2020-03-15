extern crate image;
extern crate rand;

use std::f64::consts::{E, PI, SQRT_2};
use rand::random;

mod lib;

use lib::draw_shape;

const PHI: f64 = 1.618033988749;
const ATAN_SATURATION: f64 = 1.569796;

fn main() {

    let imgx = 4000;
    let imgy = 4000;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([0]);
    }

    let iterations: u32 = imgx*imgy * 64; // arbitrary

    let cx: f64 = imgx as f64 / 2.;
    let cy: f64 = imgy as f64 / 2.;
    let mut r: f64 = 235.0;

    let mut c: f64;
    let mut c2: f64;
    let mut c3: f64;
    let mut x: u32;
    let mut y: u32;
    let mut z: f64 = 1.;
    let mut zs = vec![vec![0f64; imgy as usize]; imgx as usize];

    for i in 0..iterations {

        c = (i as f64 / iterations as f64) * PI * 2.0;
        c2 = c * E;
        c3 = c * PHI;
        x = (cx + c.sin() * r + c.cos() * z * (imgy as f64)).round() as u32 % imgx;
        y = (cy + c.cos() * r + (c2 * z).sin() * (x as f64 * z * (imgx as f64) + E.powf(c2 / c3)).sqrt()).round() as u32 % imgy;
        z = (
                //(x as f64).sin() * (i as f64) + (y as f64).cos() * 2.3f64.powf(x as f64)
                ((((x as f64).sin() * PI * (y as f64 + z)).cos() + c.ln() + (E * c.cos()) * (SQRT_2 * (y as f64).cos()) * c3.sqrt() * c2.cos()).atan() / ATAN_SATURATION)
                *
                ((((x as f64).sin() * PI * (y as f64 + z)).cos() + (c + z).powf(E) + PHI * c.cos() * (z + y as f64).powf(E) * (c3 * c2).ln() * c2.cos().powf(2.)).atan() / ATAN_SATURATION)
                //c.cos() * c.tan() * c3.cos() * (x as f64 + z.powf(c)).sin()
                * (c.powf(SQRT_2).cos() * c2.powf(3.).tan() * c3.powf(2.).cos() * (x as f64 + z.powf(c)).sin()).atan() / ATAN_SATURATION
        ).abs();
        r -= 235.0 / iterations as f64;
        zs[x as usize][y as usize] += z.powi(2);
        //draw_shape(&mut zs, x, y, z);

//        let pixel = imgbuf.get_pixel_mut(x, y);
//        let data = (*pixel as image::Luma<u8>).0;
//        *pixel = image::Luma([((data[0] as u32 + z as u32) % 255) as u8]);
        //*pixel = image::Luma([z as u8]);
    }

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([(zs[x as usize][y as usize] * 255.) as u8]);
    }

    imgbuf.save("image.png").unwrap();
}
