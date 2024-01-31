use crate::cg_math::{vector::Vector, point};

pub trait Ray {
  fn at(origin: Vector, direction: Vector, t: f64) -> Vector;
}

impl Ray for Vector {
  fn at(origin: Vector, direction: Vector, t: f64) -> Vector {
    origin.add(direction.mulc(t))
  }
}
