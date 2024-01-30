use crate::{cg_math::vector::Vector, cg_draw::draw::Drawing};

impl Vector {
  pub fn color(r: f64, g: f64, b: f64) -> Vector {
    Vector {
      x: r,
      y: g,
      z: b
    }
  }
}

impl Drawing for Vector {
  fn draw(&self, code: &mut String) {
    *code += &format!("{} {} {}\n", self.x, self.y, self.z)
  }

  fn log(&self) {
    println!("{} {} {}", self.x, self.y, self.z);
  }
}

