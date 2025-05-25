use macroquad::prelude::*;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shapes {
    pub strokes: Vec<Vec<(f32, f32)>>,
    pub rectangles: Vec<Vec<(f32, f32)>>,
    pub circles: Vec<Vec<(f32, f32)>>,
    pub lines: Vec<Vec<(f32, f32)>>,
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            strokes: vec![],
            rectangles: vec![],
            circles: vec![],
            lines: vec![],
        }
    }
}

#[macroquad::main("Client White Board")]
async fn main() {
    // Create the shared Shapes state here
    let shapes = Arc::new(Mutex::new(Shapes::new()));

    let rt = Runtime::new().unwrap();
    let shapes_reader = Arc::clone(&shapes);

    rt.spawn(async move {
        if let Err(e) = read_shapes_from_server(shapes_reader).await {
            eprintln!("Error reading from server: {:?}", e);
        }
    });

    // Main draw loop
    loop {
        clear_background(WHITE);

        // Lock and clone shapes for drawing
        let shapes_lock = shapes.lock().unwrap();
        let shapes_copy = shapes_lock.clone();

        draw_lines(&shapes_copy.lines);

        next_frame().await;
    }
}

fn draw_lines(lines: &Vec<Vec<(f32, f32)>>) {
    for line in lines {
        if line.len() == 2 {
            draw_line(line[0].0, line[0].1, line[1].0, line[1].1, 1.0, BLACK);
        } else if line.len() == 1 {
            let mouse_pos = mouse_position();
            draw_line(line[0].0, line[0].1, mouse_pos.0, mouse_pos.1, 1.0, BLACK);
        }
    }
}

// Return anyhow::Result to handle errors properly
async fn read_shapes_from_server(shared: Arc<Mutex<Shapes>>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server!");

    loop {
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;

        let mut data_buf = vec![0u8; len];
        stream.read_exact(&mut data_buf).await?;

        let shapes: Shapes = match bincode::deserialize(&data_buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Deserialization error: {:?}", e);
                continue;
            }
        };

        let mut lock = shared.lock().unwrap();
        *lock = shapes;
    }
}
