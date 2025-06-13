use serde::{ Serialize };
use macroquad::prelude::*;
use std::sync::{ Arc, Mutex };

use tokio::{
    runtime::Runtime,
    net::{ TcpListener, TcpStream },
    io::AsyncWriteExt,
    sync::Mutex as AsyncMutex,
    time::{ self, Duration },
};

use shared::*;

#[derive(Serialize, Debug, Clone)]
pub enum Mod {
    Pen,
    Rectangle,
    RectangleLines,
    Circle,
    CircleLines,
    Line,
}

#[derive(Serialize, Debug, Clone)]
pub struct Data {
    mode: Mod,
    last: Vec<(f32, f32)>,
}

pub async fn send_data(data: Data, clients: Arc<AsyncMutex<Vec<TcpStream>>>) {
    println!("---> {:?}", data);
    let encoded = match bincode::serialize(&data) {
        Ok(buf) => buf,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return;
        }
    };

    let len = encoded.len() as u32;
    let len_bytes = len.to_be_bytes(); // 4 bytes, big-endian

    let mut locked_clients = clients.lock().await;

    // Send to all clients
    for stream in locked_clients.iter_mut() {
        // First send the size
        if let Err(e) = stream.write_all(&len_bytes).await {
            eprintln!("Failed to send size to a client: {}", e);
            continue;
        }

        // Then send the actual data
        if let Err(e) = stream.write_all(&encoded).await {
            eprintln!("Failed to send data to a client: {}", e);
        }
    }
}

pub async fn update_window(
    shared_shapes: Arc<tokio::sync::Mutex<Shapes>>,
    clients: Arc<AsyncMutex<Vec<TcpStream>>>
) -> anyhow::Result<()> {
    let mut current_mod = Mod::Circle;
    let pen = load_texture("pen.png").await.unwrap();

    loop {
        clear_background(TOP_COLOR);

        // Async lock
        let mut shapes = shared_shapes.lock().await;

        listener(&mut current_mod);
        navbar(&pen);

        if is_key_down(KeyCode::Space) {
            clean_screen(&mut shapes);
        }

        let last_shape: Option<Vec<(f32, f32)>> = match current_mod {
            Mod::Pen => pen_mod(&mut shapes.strokes),
            Mod::Rectangle => get_2_point_mod(&mut shapes.rectangles),
            Mod::RectangleLines => get_2_point_mod(&mut shapes.rectangles_lines),
            Mod::Circle => get_2_point_mod(&mut shapes.circles),
            Mod::CircleLines => get_2_point_mod(&mut shapes.circles_lines),
            Mod::Line => get_2_point_mod(&mut shapes.lines),
            _ => None,
        };

        match last_shape {
            Some(last) => {
                println!("{:?}",last);
                send_data(
                    Data {
                        last,
                        mode: current_mod.clone(),
                    },
                    clients.clone()
                ).await;
            }
            None => {}
        }

        draw_all_shapes(&shapes);

        next_frame().await;
    }
}

pub const TOP: f32 = 54.0;
const TOP_COLOR: Color = WHITE;
fn pen_mod(strokes: &mut Vec<Vec<(f32, f32)>>) -> Option<Vec<(f32, f32)>> {
    /*****************************************/
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.1 > TOP {
            strokes.last_mut().unwrap().push(mouse_pos);
        } else {
            strokes.push(vec![]);
        }
    } else {
        strokes.push(vec![]);
        if strokes.len()>1{
            return Some(strokes[strokes.len() - 2].clone());
        }else{
            return Some(strokes[strokes.len() - 1].clone());
        }
    }
    return None;
}

fn get_2_point_mod(lines: &mut Vec<Vec<(f32, f32)>>) -> Option<Vec<(f32, f32)>> {
    let mouse_pos = mouse_position();
    if mouse_pos.1 < TOP {
        return None;
    }
    if lines.len() == 0 {
        lines.push(vec![]);
    }
    let mut last = lines.last_mut().unwrap();

    if is_mouse_button_down(MouseButton::Left) {
        if last.len() == 2 {
            last[1] = mouse_pos;
        } else if last.len() == 0 {
            last.push(mouse_pos);
            last.push(mouse_pos);
        }
    } else {
        if last.len() == 2 {
            lines.push(vec![]);
            return Some(lines[lines.len() - 2].clone());
        }
    }
    None
}

fn navbar(pen: &Texture2D) {
    let screen_w = screen_width();

    // Draw navbar background
    draw_rectangle(0.0, 0.0, screen_w, TOP, Color::from_rgba(250, 250, 250, 255));
    draw_line(0.0, TOP, screen_w, TOP, 2.0, GRAY);

    let button_size = 36.0;
    let padding = 16.0;
    let start_x = 20.0;
    let y = (TOP - button_size) / 2.0;

    // Pen Tool - Sky Blue
    draw_rectangle(start_x, y, button_size, button_size, Color::from_rgba(96, 165, 250, 255));
    draw_texture_ex(pen, start_x + 6.0, y + 6.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(24.0, 24.0)),
        ..Default::default()
    });

    // Rectangle Tool - Peach
    let rect_x = start_x + button_size + padding;
    draw_rectangle(rect_x, y, button_size, button_size, Color::from_rgba(255, 179, 128, 255));
    draw_rectangle(rect_x + 6.0, y + 6.0, 24.0, 24.0, Color::from_rgba(255, 87, 34, 255));

    // Rectangle Line Tool - Orange Outline
    let rect_line_x = rect_x + button_size + padding;
    draw_rectangle(rect_line_x, y, button_size, button_size, Color::from_rgba(255, 204, 153, 255));
    draw_rectangle_lines(
        rect_line_x + 6.0,
        y + 6.0,
        24.0,
        24.0,
        2.0,
        Color::from_rgba(255, 112, 67, 255)
    );

    // Circle Tool - Mint Green
    let circle_x = rect_line_x + button_size + padding;
    draw_rectangle(circle_x, y, button_size, button_size, Color::from_rgba(144, 238, 144, 255));
    draw_circle(
        circle_x + button_size / 2.0,
        y + button_size / 2.0,
        12.0,
        Color::from_rgba(0, 128, 0, 255)
    );

    // Circle Line Tool - Light Green Outline
    let circle_line_x = circle_x + button_size + padding;
    draw_rectangle(
        circle_line_x,
        y,
        button_size,
        button_size,
        Color::from_rgba(200, 255, 200, 255)
    );
    draw_circle_lines(
        circle_line_x + button_size / 2.0,
        y + button_size / 2.0,
        12.0,
        2.0,
        Color::from_rgba(0, 100, 0, 255)
    );

    // Line Tool - Lavender Purple
    let line_x = circle_line_x + button_size + padding;
    draw_rectangle(line_x, y, button_size, button_size, Color::from_rgba(221, 160, 221, 255));
    draw_line(
        line_x + 6.0,
        y + 6.0,
        line_x + 30.0,
        y + 30.0,
        3.0,
        Color::from_rgba(123, 31, 162, 255)
    );
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

// Utility function
fn point_in_rect(x: f32, y: f32, rx: f32, ry: f32, w: f32, h: f32) -> bool {
    x >= rx && x <= rx + w && y >= ry && y <= ry + h
}
