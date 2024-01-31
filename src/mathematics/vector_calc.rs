use crate::mathematics::vector::Vector;

pub trait VectorCalculation {
  fn add(&mut self, vec: Vector);
  fn mulv(&mut self, vec: Vector);
  fn mulc(&mut self, scale: f64);
  fn dot(&self, vec: Vector) -> f64;
  fn cross(&self, vec: Vector) -> Vector;
  fn len(&self) -> f64;
}
