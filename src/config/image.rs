pub struct Image {
  pub aspect_ratio: f64,
  pub image_width: u32,
  pub image_height: u32,
}

impl Image {
  pub fn set(aspect_ratio: f64, image_width: u32) -> Self {
    Self {
      aspect_ratio,
      image_width,
      image_height: (image_width as f64 / aspect_ratio) as u32,
    }
  }
}
