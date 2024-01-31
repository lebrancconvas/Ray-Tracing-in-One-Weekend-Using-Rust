use std::fs::File;
use std::io::Write;
use ray_tracing::{drawing::Drawing, mathematics::{vector_calc::VectorCalculation, Vector}, config::{image::Image, camera::Camera}};

fn main() {
    // Setting.
    let config_image = Image::set(16.0 / 9.0, 400);
    let config_camera = Camera::set((config_image.image_width / config_image.image_height) as f64, 2.0, 1.0, Vector::point(0.0, 0.0, 0.0));

    let viewport_u = Vector::new(config_camera.viewport_width, 0.0, 0.0);
    let viewport_v = Vector::new(0.0, -1.0 * config_camera.viewport_height, 0.0);

    let pixel_delta_u = viewport_u.mulc(1.0 / config_image.image_width as f64);
    let pixel_delta_v = viewport_v.mulc(1.0 / config_image.image_height as f64);

    let viewport_upper_left = config_camera.camera_center.add(viewport_u.mulc(-0.5)).add(viewport_v.mulc(-0.5)).add(Vector::new(0.0, 0.0, config_camera.focal_length));
    let pixel_00loc = ((pixel_delta_u.add(pixel_delta_v)).mulc(0.5)).add(viewport_upper_left);
    // Rendering (PPM Code Generated)
    let format = "P3".to_string();
    let width = 256;
    let height = 256;
    let color = 255;
    let mut color_code: String;

    color_code = format!("{}\n{} {}\n{}\n", format, width, height, color);
    println!("{}", color_code);

    for h in 0..255 {
        for w in 0..255 {
            let pixel_center = ((pixel_delta_u.mulc(w as f64)).add(pixel_delta_v.mulc(h as f64))).add(pixel_00loc);
        }
    }

    // Rendering (PPM File Generated)
    let mut data = File::create("image.ppm").expect("Created File Failed");
    data.write(color_code.as_bytes()).expect("Write File Failed");
    println!("Write Data Success!");
}
