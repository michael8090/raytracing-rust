pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn copy(&self) -> Vector3 {
        Vector3{..*self}
    }
    fn length(&self) -> f64 {
        (self.sqr_length()).sqrt()
    }
    fn sqr_length(&self) -> f64 {
        self.dot(self)
    }
    fn operate_with_vector(&self, v: &Vector3, operator: fn(a: f64, b: f64) -> f64) -> Vector3 {
        Vector3 {
            x: operator(self.x, v.x),
            y: operator(self.y, v.y),
            z: operator(self.z, v.z),
        }
    }
    fn operate_with_scalar(&self, v: f64, operator: fn(a: f64, b: f64) -> f64) -> Vector3 {
        Vector3 {
            x: operator(self.x, v),
            y: operator(self.y, v),
            z: operator(self.z, v),
        }
    }
    fn add(&self, v: &Vector3) -> Vector3 {
        self.operate_with_vector(v, |x: f64, y: f64| x + y)
    }
    fn substract(&self, v: &Vector3) -> Vector3 {
        self.operate_with_vector(v, |x: f64, y: f64| x - y)
    }

    fn multiply(&self, v: &Vector3) -> Vector3 {
        self.operate_with_vector(v, |x: f64, y: f64| x * y)
    }

    fn multiply_with_scalar(&self, v: f64) -> Vector3 {
        self.operate_with_scalar(v, |x: f64, y: f64| x * y)
    }

    fn normalize(&self) -> Vector3 {
        let inv = 1.0 / self.length();
        self.multiply_with_scalar(inv)
    }
    fn negate(&self) -> Vector3 {
        self.multiply_with_scalar(-1.0)
    }
    fn dot(&self, v: &Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    fn corss(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: -self.z * v.y + self.y * v.z,
            y: self.z * v.x - self.x * v.z,
            z: -self.y * v.x + self.x * v.y
        }
    }
}