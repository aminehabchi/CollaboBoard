use macroquad::prelude::*;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use std::sync::{ Arc, Mutex };
use bincode;
use std::thread;

use shared::modele::*;
use shared::drawing::*;

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

    loop {
        clear_background(WHITE);
        {
            let shapes_lock = shapes.lock().unwrap();
            let shapes_copy = shapes_lock.clone();
            drop(shapes_lock);

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

        let mut shapes = shared.lock().unwrap();

        if data.draw_mode == Type::Clean {
            shapes.clear();
            continue;
        }

        match data.mode {
            Mod::Pen => handle_pen_operation(&mut shapes.strokes, data.draw_mode, data.last),

            Mod::Rectangle =>
                handle_shape_operation(&mut shapes.rectangles, data.draw_mode, data.last),
            Mod::RectangleLines =>
                handle_shape_operation(&mut shapes.rectangles_lines, data.draw_mode, data.last),

            Mod::Circle => handle_shape_operation(&mut shapes.circles, data.draw_mode, data.last),

            Mod::CircleLines =>
                handle_shape_operation(&mut shapes.circles_lines, data.draw_mode, data.last),

            Mod::Line => handle_shape_operation(&mut shapes.lines, data.draw_mode, data.last),
        }
    }

    Ok(())
}

fn handle_pen_operation(shape: &mut Vec<Vec<(f32, f32)>>, draw_mode: Type, last_point: (f32, f32)) {
    match draw_mode {
        Type::Release => shape.push(vec![]),
        Type::Click => {
            if let Some(last) = shape.last_mut() {
                last.push(last_point);
            } else {
                shape.push(vec![]);
            }
        }
        Type::Clean => {}
    }
}

fn handle_shape_operation(
    shape: &mut Vec<Vec<(f32, f32)>>,
    draw_mode: Type,
    last_point: (f32, f32)
) {
    match draw_mode {
        Type::Release => shape.push(vec![]),
        Type::Click => {
            if let Some(last) = shape.last_mut() {
                if last.len() >= 2 {
                    last[1] = last_point;
                } else {
                    last.push(last_point);
                }
            }
        }
        Type::Clean => {}
    }
}
