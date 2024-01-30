use std::fs::File;
use std::io::Write;
use ray_tracing::{cg_draw::Drawing, cg_math::Vector};

fn main() {
    let format = "P3".to_string();
    let width = 256;
    let height = 256;
    let color = 255;
    let mut color_code: String;

    color_code = format!("{}\n{} {}\n{}\n", format, width, height, color);
    println!("{}", color_code);

    for h in 0..255 {
        for w in 0..255 {
            let pixel = Vector::color(w as f64, h as f64, 0.0);
            pixel.draw(&mut color_code);
            pixel.log();
        }
    }

    let mut data = File::create("image.ppm").expect("Created File Failed");
    data.write(color_code.as_bytes()).expect("Write File Failed");
    println!("Write Data Success!");
}
