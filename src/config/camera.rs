use crate::mathematics::vector::Vector;

pub struct Camera {
  pub viewport_height: f64,
  pub viewport_width: f64,
  pub focal_length: f64,
  pub camera_center: Vector
}

impl Camera {
  pub fn set(image_ratio: f64, viewport_height: f64, focal_length: f64, camera_center: Vector) -> Self {
    Self {
      viewport_height,
      viewport_width: (viewport_height / image_ratio),
      focal_length,
      camera_center
    }
  }
}
