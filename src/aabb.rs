
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
pub struct Point {
    pub x: f64, pub y: f64,
}
pub fn intersects(bound: &Bounds, point: &Point) -> bool {
    bound.x < point.x &&
    bound.y < point.y &&
    point.x - bound.x < bound.w &&
    point.y - bound.y < bound.h
}
