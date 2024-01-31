use crate::mathematics::vector_calc::VectorCalculation;

pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z }
  }
}

impl VectorCalculation for Vector {
  fn add(&self, vec: Vector) -> Vector {
    Vector {
      x: self.x + vec.x,
      y: self.y + vec.y,
      z: self.z + vec.z
    }
  }

  fn mulv(&self, vec: Vector) -> Vector {
    Vector {
      x: self.x * vec.x,
      y: self.y * vec.y,
      z: self.z * vec.z
    }
  }

  fn mulc(&self, scale: f64) -> Vector {
    Vector {
      x: self.x * scale,
      y: self.y * scale,
      z: self.z * scale
    }
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

