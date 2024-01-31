use crate::cg_math::vector::Vector;

impl Vector {
  pub fn point(x: f64, y: f64, z: f64) -> Vector {
    Vector {
      x,
      y,
      z
    }
  }
}
