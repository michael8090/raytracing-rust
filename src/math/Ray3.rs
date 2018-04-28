mod Vector3;

use Vector3::Vector3;

pub struct Ray3 {
    origin: Vector3,
    direction: Vector3,
}

impl Ray3 {
    fn get_point(&self, t: f64) -> Vector3 {
        self.origin
    }
}