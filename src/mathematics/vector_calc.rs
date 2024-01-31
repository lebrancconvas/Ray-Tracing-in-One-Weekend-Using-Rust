use crate::mathematics::vector::Vector;

pub trait VectorCalculation {
  fn add(&self, vec: Vector) -> Vector;
  fn mulv(&self, vec: Vector) -> Vector;
  fn mulc(&self, scale: f64) -> Vector;
  fn dot(&self, vec: Vector) -> f64;
  fn cross(&self, vec: Vector) -> Vector;
  fn len(&self) -> f64;
}
