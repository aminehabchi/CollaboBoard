use macroquad::prelude::*;

enum Mod {
    Pen,
    Rectangle,
}
const TOP:f32=54.0;

#[macroquad::main("White Board")]
async fn main() {
    // Load the pen texture once
    let pen = load_texture("pen.png").await.unwrap();
    let top_color = Color::from_rgba(220, 95, 0, 255);

    let mut strokes: Vec<Vec<(f32, f32)>> = vec![vec![]];
    let mut rectangles: Vec<Vec<(f32, f32)>> = vec![vec![]];

    let mut current_mod:Mod=Mod::Pen;

    loop {
            clear_background(top_color);
        /*****************************************/
            navbar(&pen);
        /*****************************************/
        match current_mod{
            Mod::Pen=>pen_mod(&mut strokes),
            Mod::Rectangle=>rectangle_mod(),
        }
        /*****************************************/
            draw_strokes(&strokes);
            // draw_rectangles(&rectangles);
        /*****************************************/
            next_frame().await;
    }
}

fn rectangle_mod(){
    
//    if is_mouse_button_released(MouseButton::Left){
//         let mouse_pos = mouse_position();
//         if mouse_pos.1>TOP{
//             if rectangles[rectangles.len()-1].len()==2{
//                 rectangles[rectangles.len()-1].push(mouse_pos);
//             }else{
//                 rectangles.push(vec![]);
//             }
//         }
//    }
}

// fn draw_rectangles(rectangles: &Vec<Vec<(f32, f32)>>){
//     for rec in rectangles{

//     }
// }

fn pen_mod(strokes :&mut Vec<Vec<(f32, f32)>>){
    /*****************************************/
    if is_key_down(KeyCode::Space) {
        *strokes = vec![vec![]];
    }
    /*****************************************/
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

fn navbar(pen: &Texture2D) {

    // Draw pen button
    draw_texture(pen, 15.0, 15.0, WHITE);

    // Draw rectangle button
    draw_rectangle(44.0, 15.0, 24.0, 24.0, BLACK);

    // Draw white rec
    draw_rectangle(0.0,TOP, screen_width(), screen_height()-TOP, WHITE);

    // Draw circle
    draw_circle(88.0,27.0,12.0,BLACK);
}
