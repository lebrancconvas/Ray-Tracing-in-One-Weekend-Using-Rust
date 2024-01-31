pub struct Config {
  aspect_ratio: f64,
  image_width: u32,
  image_height: u32,
  viewport_width: u32,
  viewport_height: u32
}

impl Config {
  pub fn set(aspect_ratio: f64, image_width: u32, viewport_width: u32) -> Config {
    Config {
      aspect_ratio,
      image_width,
      image_height: (image_width as f64 / aspect_ratio) as u32,
      viewport_width,
      viewport_height: (viewport_height / (image_width as f64 / image_height as f64)) as u32
    }
  }
}
