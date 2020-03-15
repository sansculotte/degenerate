struct Engine {
    r: f64,
    c: f64,
    c2: f64,
    c3: f64,
    x: u32,
    y: u32,
    z: f64,
    zs: std::vec::Vec<std::vec::Vec<f64>>
}


pub struct Dings {
    imgx: u32,
    imgy: u32,
    cx: f64,
    cy: f64,
    engine: Engine
}

impl Default for Dings {
    fn default() -> Dings {
        Dings {
            imgx: 4000,
            imgy: 4000,
            cx: imgx as f64 / 2,
            cy: imgy as f64 / 2,
            engine: {
                r: 235.0,
                x: 0.,
                y: 0.,
                z: 0.,
                zs: vec![vec![0f64; Self.imgy as usize]; Self.imgx as usize]
            } as Engine
        }
    }
}
