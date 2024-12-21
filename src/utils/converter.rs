use core::panic;
use std::{fs::OpenOptions, io::Write};

use image::GenericImageView;

pub struct Converter {
    pub image_path: String,
    pixels: Vec<String>,
}

impl Converter {
    pub fn new(path: String) -> Converter {
        return Converter {
            image_path: path,
            pixels: Vec::new(),
        };
    }

    pub fn convert(&mut self) {
        println!("Reading image from path: {}", self.image_path);

        let image: image::DynamicImage = match image::open(&self.image_path) {
            Ok(img) => img,
            Err(e) => panic!("Error reading image: {}", e),
        };

        let (width, _height) = image.dimensions();

        for (x, _y, pixel) in image.pixels() {
            let mut hex: String = format!(
                "#{:02X}{:02X}{:02X}{:02X} ",
                pixel[0], pixel[1], pixel[2], pixel[3]
            );

            if x == width - 1 {
                hex.push('\n');
            }

            self.pixels.push(hex);
        }

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open("output.quiche")
        {
            Ok(f) => f,
            Err(e) => panic!("Error opening file: {}", e),
        };

        if let Err(e) = file.write_all(self.pixels.join("").as_bytes()) {
            panic!("Error writing to file: {}", e);
        }
    }
}
