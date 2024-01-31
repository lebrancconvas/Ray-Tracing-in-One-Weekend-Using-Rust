use crate::mathematics::vector_calc::VectorCalculation;

pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl VectorCalculation for Vector {
  fn add(&mut self, vec: Vector) {
    self.x += vec.x;
    self.y += vec.y;
    self.z += vec.z;
  }

  fn mulv(&mut self, vec: Vector) {
    self.x *= vec.x;
    self.y *= vec.y;
    self.z *= vec.z;
  }

  fn mulc(&mut self, scale: f64) {
    self.x *= scale;
    self.y *= scale;
    self.z *= scale;
  }

  fn dot(&self, vec: Vector) -> f64 {
    (self.x * vec.x) + (self.y * vec.y) + (self.z * vec.z)
  }

  fn cross(&self, vec: Vector) -> Vector {
    Vector {
      x: (self.y * vec.z) - (self.z * vec.y),
      y: (self.z * vec.x) - (self.x * vec.z),
      z: (self.x * vec.y) - (self.y * vec.x)
    }
  }

  fn len(&self) -> f64 {
    ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
  }
}

