use std::fs::OpenOptions;
use std::io::Read;

use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::{glutin, implement_vertex, program, uniform, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

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

        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Glium - ESC to exit")
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let content = self.content.clone(); // Clonar el contenido para moverlo al cierre

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            in vec3 color;
            out vec3 vColor;
            void main() {
                vColor = color;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec3 vColor;
            out vec4 f_color;
            void main() {
                f_color = vec4(vColor, 1.0);
            }
        "#;

        let program = program!(&display,
            140 => {
                vertex: vertex_shader_src,
                fragment: fragment_shader_src,
            },
        )
        .unwrap();

        event_loop.run(move |event, _, control_flow| match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => (),
            },
            glutin::event::Event::RedrawRequested(_) => {
                let mut target = display.draw();
                target.clear_color(1.0, 1.0, 1.0, 1.0);

                let mut vertices = Vec::new();

                for (y, line) in content.iter().enumerate() {
                    for (x, pixel) in line.iter().enumerate() {
                        if pixel.len() > 1 {
                            if let Ok(color) = u32::from_str_radix(&pixel[1..], 16) {
                                let red = ((color >> 16) & 0xFF) as f32 / 255.0;
                                let green = ((color >> 8) & 0xFF) as f32 / 255.0;
                                let blue = (color & 0xFF) as f32 / 255.0;

                                let x_pos = x as f32 / width as f32 * 2.0 - 1.0;
                                let y_pos = 1.0 - y as f32 / height as f32 * 2.0;

                                vertices.push(Vertex {
                                    position: [x_pos, y_pos],
                                    color: [red, green, blue],
                                });
                            }
                        }
                    }
                }

                let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
                let indices = NoIndices(PrimitiveType::Points);

                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniform! {},
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();
            }
            _ => (),
        });
    }
}
