use piston_window::{clear, rectangle, G2d, PistonWindow, WindowSettings};
use std::{fs::OpenOptions, io::Read};

pub struct Reader {
    content: Vec<Vec<String>>,
    file_path: String,
}

impl Reader {
    pub fn new(file_path: String) -> Reader {
        Reader {
            content: vec![vec![]],
            file_path,
        }
    }

    pub fn read(&mut self) -> String {
        let mut file = match OpenOptions::new().read(true).open(&self.file_path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening file: {}", e),
        };

        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {
                self.content = buffer
                    .split("\n")
                    .map(|line| line.split(" ").map(|s| s.to_string()).collect())
                    .collect();

                buffer
            }
            Err(e) => panic!("Error reading file: {}", e),
        }
    }

    pub fn print(&self) {
        let height = (self.content.len() - 1) as u32;
        let width = (self.content[0].len() - 1) as u32;

        println!("Height: {}, Width: {}", height, width);

        let mut window: PistonWindow = WindowSettings::new("Canvas - ESC to exit", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();

        while let Some(event) = window.next() {
            window.draw_2d(&event, |c: piston_window::Context, g: &mut G2d, _| {
                clear([1.0; 4], g);

                for (y, line) in self.content.iter().enumerate() {
                    for (x, pixel) in line.iter().enumerate() {
                        if pixel.len() > 1 {
                            if let Ok(color) = u32::from_str_radix(&pixel[1..], 16) {
                                let red = ((color >> 16) & 0xFF) as f32 / 255.0;
                                let green = ((color >> 8) & 0xFF) as f32 / 255.0;
                                let blue = (color & 0xFF) as f32 / 255.0;
                                rectangle(
                                    [red, green, blue, 1.0],
                                    [x as f64, y as f64, 1.0, 1.0],
                                    c.transform,
                                    g,
                                );
                            }
                        }
                    }
                }
            });
        }
    }
}
