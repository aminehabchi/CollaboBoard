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
}

#[derive(Deserialize, Debug, Clone)]
pub enum Mod {
    Pen,
    Rectangle,
    RectangleLines,
    Circle,
    CircleLines,
    Line,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    mode: Mod,
    last: Vec<(f32, f32)>,
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
            // draw_rectangles_lines(&shapes_copy.rectangles_lines);
            draw_circles(&shapes_copy.circles);
            // draw_circles_lines(&shapes_copy.circles_lines);
        }

        next_frame().await;
    }
}

async fn read_shapes_from_server(shared: Arc<Mutex<Shapes>>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server!");

    loop {
        // Read the length first
        let mut len_buf = [0u8; 4];
        match stream.read_exact(&mut len_buf).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading length: {:?}", e);
                break;
            }
        }
        let len = u32::from_be_bytes(len_buf) as usize;

        // Validate length to prevent excessive memory allocation
        if len > 1024 * 1024 {
            // 1MB limit
            eprintln!("Received suspiciously large data length: {}", len);
            break;
        }

        // Read the actual data
        let mut data_buf = vec![0u8; len];
        match stream.read_exact(&mut data_buf).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading data: {:?}", e);
                break;
            }
        }

        // Deserialize the data
        let data: Data = match bincode::deserialize(&data_buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Deserialization error: {:?}", e);
                continue;
            }
        };

        println!("Received data: {:?}", data);

        // Update the shared shape state
        {
            let mut shapes = shared.lock().unwrap();
            match data.mode {
                Mod::Pen => shapes.strokes.push(data.last),
                Mod::Rectangle => shapes.rectangles.push(data.last),
                Mod::RectangleLines => shapes.rectangles_lines.push(data.last),
                Mod::Circle => shapes.circles.push(data.last),
                Mod::CircleLines => shapes.circles_lines.push(data.last),
                Mod::Line => shapes.lines.push(data.last),
            }
        }
    }

    Ok(())
}
