use macroquad::prelude::*;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use std::sync::{ Arc, Mutex };
use serde::{ Serialize, Deserialize };
use bincode;
use std::thread;

mod drawing;
use drawing::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shapes {
    pub strokes: Vec<Vec<(f32, f32)>>,
    pub rectangles: Vec<Vec<(f32, f32)>>,
    pub rectangles_lines: Vec<Vec<(f32, f32)>>,
    pub circles: Vec<Vec<(f32, f32)>>,
    pub circles_lines: Vec<Vec<(f32, f32)>>,
    pub lines: Vec<Vec<(f32, f32)>>,
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            strokes: vec![],
            rectangles: vec![],
            rectangles_lines: vec![],
            circles: vec![],
            circles_lines: vec![],
            lines: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.strokes.clear();
        self.rectangles.clear();
        self.rectangles_lines.clear();
        self.circles.clear();
        self.circles_lines.clear();
        self.lines.clear();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Mod {
    Pen,
    Rectangle,
    RectangleLines,
    Circle,
    CircleLines,
    Line,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Type {
    Release,
    Clean,
    Click,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Data {
    mode: Mod,
    draw_mode: Type,
    last: (f32, f32),
}

#[macroquad::main("Client White Board")]
async fn main() {
    let shapes = Arc::new(Mutex::new(Shapes::new()));
    let shapes_reader = Arc::clone(&shapes);

    // Spawn a separate thread for network operations
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = read_shapes_from_server(shapes_reader).await {
                eprintln!("Error reading from server: {:?}", e);
            }
        });
    });

    // Main rendering loop
    loop {
        clear_background(WHITE);

        // Render the shapes
        {
            let shapes_lock = shapes.lock().unwrap();
            let shapes_copy = shapes_lock.clone();
            drop(shapes_lock); // Release the lock early

            draw_lines(&shapes_copy.lines);
            draw_strokes(&shapes_copy.strokes);
            draw_rectangles(&shapes_copy.rectangles);
            draw_rectangles_lines(&shapes_copy.rectangles_lines);
            draw_circles(&shapes_copy.circles);
            draw_circles_lines(&shapes_copy.circles_lines);
        }

        next_frame().await;
    }
}

pub async fn read_shapes_from_server(shared: Arc<Mutex<Shapes>>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server!");

    loop {
        // Read the length first
        let mut len_buf = [0u8; 4];
        if stream.read_exact(&mut len_buf).await.is_err() {
            eprintln!("Failed to read length");
            break;
        }
        let len = u32::from_be_bytes(len_buf) as usize;

        if len > 1024 * 1024 {
            eprintln!("Payload too large: {}", len);
            break;
        }

        let mut data_buf = vec![0u8; len];
        if stream.read_exact(&mut data_buf).await.is_err() {
            eprintln!("Failed to read data buffer");
            break;
        }

        let data: Data = match bincode::deserialize(&data_buf) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Deserialization error: {:?}", e);
                continue;
            }
        };

        println!("Received data: {:?}", data);

        let mut shapes = shared.lock().unwrap();

        match data.mode {
            Mod::Pen => {
                match data.draw_mode {
                    Type::Release => shapes.strokes.push(vec![]),
                    Type::Click => {
                        if let Some(last) = shapes.strokes.last_mut() {
                            last.push(data.last);
                        } else {
                            shapes.strokes.push(vec![]);
                        }
                    }
                    Type::Clean => {
                        shapes.clear();
                    }
                }
            }
            Mod::Rectangle => {
                match data.draw_mode {
                    Type::Release => shapes.rectangles.push(vec![]),
                    Type::Click => {
                        if let Some(last) = shapes.rectangles.last_mut() {
                            if last.len() >= 2 {
                                last[1] = data.last;
                            } else {
                                last.push(data.last);
                            }
                        }
                    }
                    Type::Clean => {
                        shapes.clear();
                    }
                }
            }
            Mod::RectangleLines => {
                match data.draw_mode {
                    Type::Release => shapes.rectangles_lines.push(vec![]),
                    Type::Click => {
                        if let Some(last) = shapes.rectangles_lines.last_mut() {
                            if last.len() >= 2 {
                                last[1] = data.last;
                            } else {
                                last.push(data.last);
                            }
                        }
                    }
                    Type::Clean => {
                        shapes.clear();
                    }
                }
            }
            Mod::Circle => {
                match data.draw_mode {
                    Type::Release => shapes.circles.push(vec![]),
                    Type::Click => {
                        if let Some(last) = shapes.circles.last_mut() {
                            if last.len() >= 2 {
                                last[1] = data.last;
                            } else {
                                last.push(data.last);
                            }
                        }
                    }
                    Type::Clean => {
                        shapes.clear();
                    }
                }
            }
            Mod::CircleLines => {
                match data.draw_mode {
                    Type::Release => shapes.circles_lines.push(vec![]),
                    Type::Click => {
                        if let Some(last) = shapes.circles_lines.last_mut() {
                            if last.len() >= 2 {
                                last[1] = data.last;
                            } else {
                                last.push(data.last);
                            }
                        }
                    }
                    Type::Clean => {
                        shapes.clear();
                    }
                }
            }
            Mod::Line => {
                match data.draw_mode {
                    Type::Release => shapes.lines.push(vec![]),
                    Type::Click => {
                        if let Some(last) = shapes.lines.last_mut() {
                            if last.len() >= 2 {
                                last[1] = data.last;
                            } else {
                                last.push(data.last);
                            }
                        }
                    }
                    Type::Clean => {
                        shapes.clear();
                    }
                }
            }
        }
    }

    Ok(())
}
