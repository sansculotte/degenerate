#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct Feed {
    pub p1: Point,
    pub p2: Point,
    pub radius: f64,
}
