use std::fs::File;
use std::io::Write;

fn main() {
    let format = "P3".to_string();
    let width = "255".to_string();
    let height = "255".to_string();
    let color = "255".to_string();
    let mut color_code: String;

    color_code = format!("{}\n{} {}\n{}\n", format, width, height, color);
    println!("{}", color_code);

    for h in 0..255 {
        for w in 0..255 {
            let r = h;
            let g = h;
            let b = w;
            color_code += &format!("{} {} {}\n", r, g, b);
            println!("{} {} {}", r, g, b);
        }
    }

    let mut data = File::create("app.ppm").expect("Created File Failed");
    data.write(color_code.as_bytes()).expect("Write File Failed");
    println!("Write Data Success!");
}