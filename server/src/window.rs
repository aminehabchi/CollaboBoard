use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

use crate::Shapes;

pub enum Mod {
    Pen,
    Rectangle,
    Circle,
    Line,
}

pub const TOP: f32 = 54.0;
const TOP_COLOR: Color = WHITE; // Or any color you want

pub async fn update_window(shared_shapes:&mut Arc<Mutex<Shapes>>) -> anyhow::Result<()> {
    let current_mod = Mod::Line;

    loop {
            clear_background(TOP_COLOR);
        /*****************************************/
        let mut shapes = shared_shapes.lock().unwrap();
        /*****************************************/
            if is_key_down(KeyCode::Space) {
                clean_screen(&mut shapes);
            }
        /*****************************************/

        match current_mod {
            Mod::Line => {
                get_2_point_mod(&mut shapes.lines);
            }
            _ => {}
        }

        draw_lines(&shapes.lines);

        next_frame().await;
    }
}


fn get_2_point_mod(lines: &mut Vec<Vec<(f32, f32)>>) {
    if is_mouse_button_released(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.1 > TOP {
            match lines.last_mut() {
                Some(last_line) if last_line.len() == 1 => {
                    last_line.push(mouse_pos);
                }
                _ => {
                    lines.push(vec![mouse_pos]);
                }
            }
        }
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

fn clean_screen(shapes :&mut Shapes){
    shapes.strokes=vec![vec![]];
    shapes.rectangles=vec![vec![]];
    shapes.circles=vec![vec![]];
    shapes.lines=vec![vec![]];
}
