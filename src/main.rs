use macroquad::prelude::*;

enum Mod {
    Pen,
    Rec,
}
const TOP:f32=54.0;

#[macroquad::main("White Board")]
async fn main() {
    // Load the pen texture once
    let pen = load_texture("pen.png").await.unwrap();
    let top_color = Color::from_rgba(220, 95, 0, 255);

    let mut strokes: Vec<Vec<(f32, f32)>> = vec![vec![]];

    loop {
        clear_background(top_color);
        /*****************************************/
        set_up(&pen);
        /*****************************************/

        if is_key_down(KeyCode::Space) {
            strokes = vec![vec![]];
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            if mouse_pos.1>TOP{
                strokes.last_mut().unwrap().push(mouse_pos);
            }else{
                strokes.push(vec![]);
            }
        } else if !strokes.last().unwrap().is_empty() {
            strokes.push(vec![]);
        }

        /*****************************************/
        draw_strokes(&strokes);
        /*****************************************/
        next_frame().await;
    }
}


fn draw_strokes(strokes: &Vec<Vec<(f32, f32)>>){
    for stroke in strokes {
        for i in 1..stroke.len() {
            let (x1, y1) = stroke[i - 1];
            let (x2, y2) = stroke[i];
            draw_line(x1, y1, x2, y2, 2.0, BLACK);
        }
    }
}

fn set_up(pen: &Texture2D) {

    // Draw pen button
    draw_texture(pen, 15.0, 15.0, WHITE);

    // Draw rectangle button
    draw_rectangle(44.0, 15.0, 24.0, 24.0, BLACK);


    // Draw white rec
    draw_rectangle(0.0,TOP, screen_width(), screen_height()-TOP, WHITE);
}
