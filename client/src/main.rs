use macroquad::prelude::*;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use bincode;


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
        /*****************************************/
            let shapes_lock = shapes.lock().unwrap();
            let shapes_copy = shapes_lock.clone();
        /*****************************************/
            draw_lines(&shapes_copy.lines);
            draw_strokes(&shapes_copy.strokes);
            draw_rectangles(&shapes_copy.rectangles);
            draw_circles(&shapes_copy.circles);
        /*****************************************/
        next_frame().await;
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

