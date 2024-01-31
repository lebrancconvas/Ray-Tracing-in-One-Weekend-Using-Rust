pub struct Config {
  aspect_ratio: f64,
  image_width: u32,
  image_height: u32,
  viewport_width: u32,
  viewport_height: u32
}

impl Config {
  pub fn set(&mut self, aspect_ratio: f64, image_width: u32, viewport_height: u32) {
    self.aspect_ratio = aspect_ratio;
    self.image_width = image_width;
    self.image_height = (image_width as f64 / aspect_ratio) as u32;
    self.viewport_height = viewport_height;
    self.viewport_width = viewport_height * (image_width as f64 / image_height as f64) as u32;
  }
}
