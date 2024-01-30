pub trait Drawing {
  fn draw(&self, code: &mut String);
  fn log(&self);
}
