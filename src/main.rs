use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

use sunray::{get_image_ppm, Scene};

fn main() {
    let start = Instant::now();

    let path = Path::new("image.ppm");
    let display = path.display();

    let result = get_image_ppm(Scene::Knot);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {}
    }

    // Yeah it's a bit dumb to open the image again
    let img = image::open("image.ppm").unwrap();
    img.save("image.png").unwrap();

    println!("{:.2?} seconds to run.", start.elapsed());
}
