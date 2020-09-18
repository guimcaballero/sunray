use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod vec3;
use vec3::*;
mod ray;
use ray::*;

fn main() {
    let path = Path::new("image.ppm");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let result = get_image_string();

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn get_image_string() -> String {
    let image_width = 256;
    let image_height = 256;

    let mut result = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        println!("Remaining scanlines: {}", j);
        for i in 0..image_width {
            let color = Color {
                x: (i as f32) / (image_width - 1) as f32,
                y: (j as f32) / (image_width - 1) as f32,
                z: 0.25,
            };

            color.write_color(&mut result);
        }
    }

    return result;
}
