extern crate image;

mod ghostweb;

use ghostweb::ghostweb;

const PHI: f64 = 1.618033988749;
const ATAN_SATURATION: f64 = 1.569796;

fn main() {

    let width = 4000;
    let height = 4000;

    // Create a new ImgBuf with width: width and height: height
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    // and set all to black
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([0]);
    }

    let zs = ghostweb(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([(zs[x as usize][y as usize] * 255.) as u8]);
    }
    image::imageops::blur(&imgbuf, 5.);

    imgbuf.save("image.png").unwrap();
}
