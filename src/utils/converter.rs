use image::GenericImageView;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Converter {
    image_path: String,
    pixels: Vec<String>,
}

impl Converter {
    pub fn new(path: String) -> Converter {
        Converter {
            image_path: path,
            pixels: Vec::new(),
        }
    }

    pub fn convert(&mut self) {
        println!("Reading image from path: {}", self.image_path);

        let image: image::DynamicImage = match image::open(&self.image_path) {
            Ok(img) => img,
            Err(e) => panic!("Error reading image: {}", e),
        };

        let (width, _height) = image.dimensions();

        for (x, _y, pixel) in image.pixels() {
            let hex: String = format!("#{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2]);

            // Add a newline character at the end of each row
            if x == width - 1 {
                self.pixels.push(format!("{}\n", hex));
            } else {
                self.pixels.push(hex);
            }
        }

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("output.quiche")
        {
            Ok(f) => f,
            Err(e) => panic!("Error opening file: {}", e),
        };

        if let Err(e) = file.write_all(self.pixels.join(" ").as_bytes()) {
            panic!("Error writing to file: {}", e);
        }
    }
}
