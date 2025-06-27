use macroquad::prelude::*;
use std::sync::{ Arc };

use tokio::{
    net::{  TcpStream },
    io::AsyncWriteExt,
    sync::Mutex as AsyncMutex,
};

use shared::modele::*;
use shared::drawing::*;


pub async fn send_data(data: Data, clients: Arc<AsyncMutex<Vec<TcpStream>>>) {
    let encoded = match bincode::serialize(&data) {
        Ok(buf) => buf,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return;
        }
    };

    let len = encoded.len() as u32;
    let len_bytes = len.to_be_bytes(); 

    let mut locked_clients = clients.lock().await;

    for stream in locked_clients.iter_mut() {
        if let Err(e) = stream.write_all(&len_bytes).await {
            eprintln!("Failed to send size to a client: {}", e);
            continue;
        }

        if let Err(e) = stream.write_all(&encoded).await {
            eprintln!("Failed to send data to a client: {}", e);
        }
    }
}

pub async fn update_window(
    shared_shapes: Arc<tokio::sync::Mutex<Shapes>>,
    clients: Arc<AsyncMutex<Vec<TcpStream>>>
) -> anyhow::Result<()> {
    let mut current_mod = Mod::Rectangle;
    let pen = load_texture("pen.png").await.unwrap();

    loop {
        clear_background(TOP_COLOR);

        // Async lock
        let mut shapes = shared_shapes.lock().await;

        listener(&mut current_mod);
        navbar(&pen);

        if is_key_down(KeyCode::Space) {
            send_data(
                Data {
                    mode: current_mod.clone(),
                    draw_mode: Type::Clean,
                    last: (1.0, 0.0),
                },
                clients.clone()
            ).await;
            clean_screen(&mut shapes);
        }

        let last_shape: Option<Data> = match current_mod {
            Mod::Pen => pen_mod(&mut shapes.strokes, current_mod.clone()),
            Mod::Rectangle => get_2_point_mod(&mut shapes.rectangles, current_mod.clone()),
            Mod::RectangleLines =>
                get_2_point_mod(&mut shapes.rectangles_lines, current_mod.clone()),
            Mod::Circle => get_2_point_mod(&mut shapes.circles, current_mod.clone()),
            Mod::CircleLines => get_2_point_mod(&mut shapes.circles_lines, current_mod.clone()),
            Mod::Line => get_2_point_mod(&mut shapes.lines, current_mod.clone()),
        };

        match last_shape {
            Some(mut data) => {
                data.mode = current_mod.clone();
                send_data(data, clients.clone()).await;
            }
            None => {}
        }

        draw_all_shapes(&shapes);

        next_frame().await;
    }
}

pub const TOP: f32 = 54.0;
const TOP_COLOR: Color = WHITE;
fn pen_mod(strokes: &mut Vec<Vec<(f32, f32)>>, mode: Mod) -> Option<Data> {
    if strokes.is_empty() {
        strokes.push(vec![]);
        return None;
    }
    /*****************************************/
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.1 > TOP {
            strokes.last_mut().unwrap().push(mouse_pos);
            return Some(Data { mode: mode.clone(), draw_mode: Type::Click, last: mouse_pos });
        }
    } else {
        if strokes.last_mut().unwrap().len() > 0 {
            strokes.push(vec![]);
            return Some(Data { mode: mode.clone(), draw_mode: Type::Release, last: (0.0, 0.0) });
        }
    }
    return None;
}

pub fn get_2_point_mod(shapes: &mut Vec<Vec<(f32, f32)>>, mode: Mod) -> Option<Data> {
    let mouse_pos = mouse_position();

    if mouse_pos.1 < TOP {
        return None;
    }

    let left_down = is_mouse_button_down(MouseButton::Left);

    if shapes.is_empty() {
        shapes.push(vec![]);
        return Some(Data { mode: mode.clone(), draw_mode: Type::Release, last: mouse_pos });
    }

    // Handle mouse down case
    if left_down {
        let last = shapes.last_mut().unwrap();
        match last.len() {
            0 => {
                last.push(mouse_pos);
                last.push(mouse_pos);
                Some(Data { mode: mode.clone(), draw_mode: Type::Release, last: mouse_pos })
            }
            1 => {
                last.push(mouse_pos);
                Some(Data { mode: mode.clone(), draw_mode: Type::Click, last: mouse_pos })
            }
            2 => {
                last[1] = mouse_pos;
                Some(Data { mode: mode.clone(), draw_mode: Type::Click, last: mouse_pos })
            }
            _ => None,
        }
    } else {
        // Handle mouse up case - check if we have a complete shape
        if let Some(last) = shapes.last() {
            if last.len() == 2 {
                let last_point = last[1];
                shapes.push(vec![]);
                return Some(Data { mode: mode.clone(), draw_mode: Type::Click, last: last_point });
            }
        }
        None
    }
}

fn listener(current_mod: &mut Mod) {
    let (x, y) = mouse_position();

    // Ignore clicks below the navbar
    if y > TOP {
        return;
    }

    let button_size = 36.0;
    let padding = 16.0;
    let start_x = 20.0;
    let top_y = (TOP - button_size) / 2.0;

    let mut bx = start_x;

    // Pen Tool
    if point_in_rect(x, y, bx, top_y, button_size, button_size) {
        *current_mod = Mod::Pen;
        return;
    }

    // Rectangle Tool
    bx += button_size + padding;
    if point_in_rect(x, y, bx, top_y, button_size, button_size) {
        *current_mod = Mod::Rectangle;
        return;
    }

    // Rectangle Lines Tool
    bx += button_size + padding;
    if point_in_rect(x, y, bx, top_y, button_size, button_size) {
        *current_mod = Mod::RectangleLines;
        return;
    }

    // Circle Tool
    bx += button_size + padding;
    let cx = bx + button_size / 2.0;
    let cy = top_y + button_size / 2.0;
    if (x - cx).powi(2) + (y - cy).powi(2) <= (12.0f32).powi(2) {
        *current_mod = Mod::Circle;
        return;
    }

    // Circle Lines Tool
    bx += button_size + padding;
    let cx = bx + button_size / 2.0;
    let cy = top_y + button_size / 2.0;
    if (x - cx).powi(2) + (y - cy).powi(2) <= (12.0f32).powi(2) {
        *current_mod = Mod::CircleLines;
        return;
    }

    // Line Tool
    bx += button_size + padding;
    if point_in_rect(x, y, bx, top_y, button_size, button_size) {
        *current_mod = Mod::Line;
        return;
    }
}

fn point_in_rect(x: f32, y: f32, rx: f32, ry: f32, w: f32, h: f32) -> bool {
    x >= rx && x <= rx + w && y >= ry && y <= ry + h
}
