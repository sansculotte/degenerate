/*
pub fn new_x_001(cs: f64, x: u32, y: u32, z:f64, c:f64) -> u32 {
    (cx + c.sin() * r + (c * 1.777).cos() * z * (imgy as f64)).round() as u32
}

pub fn new_y_001(cs: f64, x: u32, y: u32, z:f64, c:f64) -> u32 {
    (cy + c.cos() * r + (c * 1.666).sin() * ((x as f64 * z * (imgx as f64))).sqrt() as f64).round() as u32
}
*/

fn spiral(n: i32) -> (i32, i32) {
    let k: i32 = (((n as f64).sqrt() - 1.) / 2.).ceil() as i32;
    let t0: i32 = 2 * k + 1;
    let mut m: i32 = t0 * t0;
    let t: i32 = t0 - 1;

    if n >= m - t {
        return (k - (m - n), -k)
    }
    else {
        m = m - t;
    }

    if n >= m - t {
        return (-k, (m - n) - k)
    }
    else {
        m = m - t;
    }

    if n >= m - t {
        return ((m - n) - k, k)
    }

    return (k, k-(m-n-t))

}

pub fn draw_shape(buffer: &mut Vec<Vec<f64>>, center_x: u32, center_y: u32, z: f64) {
    let iterations: u32 = (5. * z).ceil() as u32;
    let max_x = buffer.len();
    let max_y = buffer[0].len();

    for i in 0..iterations {
        let (spiral_x, spiral_y) = spiral(i as i32);
        let x: i32 = (center_x as i32) + spiral_x;
        let y: i32 = (center_y as i32) + spiral_y;
        if x < max_x as i32 && y < max_y as i32 && x >= 0 && y >= 0 {
            let shade: f64 = if i == 0 { 1. } else { iterations as f64 / i as f64 };
            buffer[x as usize][y as usize] += z * shade;
        }
    }
}
