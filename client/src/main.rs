use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use shared::Action;
use tokio::sync::mpsc;
use tokio::{runtime::Runtime, task};
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

enum Mod {
    Pen,
    Rectangle,
    Circle,
    Line,
}

const TOP:f32=54.0;

#[macroquad::main("White Board")]
async fn main() {
    // Load the pen texture once
    let pen = load_texture("pen.png").await.unwrap();
    let top_color = Color::from_rgba(255, 197, 0, 255);

    let (tx, mut rx) = mpsc::unbounded_channel::<Action>();

    // let mut strokes: Vec<Vec<(f32, f32)>> = vec![vec![]];
    let mut strokes = Arc::new(Mutex::new(vec![vec![]]));
    let remote_strokes = Arc::new(Mutex::<Vec<Vec<(f32, f32)>>>::new(vec![]));
    let remote_clone = remote_strokes.clone();

    let mut rectangles: Vec<Vec<(f32, f32)>> = vec![vec![]];
    let mut circles: Vec<Vec<(f32, f32)>> = vec![vec![]];
    let mut lines: Vec<Vec<(f32, f32)>> = vec![vec![]];

    let mut current_mod:Mod=Mod::Circle;

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        // connect to server
        let stream = TcpStream::connect("127.0.0.1:4000").await.unwrap();
        let (reader, mut writer) = stream.into_split();

        // Task 1: Send user actions to server
        let mut tx_reader = rx;
        let mut writer = BufWriter::new(writer);
        task::spawn(async move {
            while let Some(action) = tx_reader.recv().await {
                let serialized = serde_json::to_string(&action).unwrap();
                writer.write_all(serialized.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();
                writer.flush().await.unwrap();
            }
        });

        // Task 2: Read remote actions from server
        let mut lines = BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if let Ok(action) = serde_json::from_str::<Action>(&line) {
                if let Action::Stroke(points) = action {
                    remote_clone.lock().unwrap().push(points);
                }
            }
        }
    });

    loop {
            clear_background(top_color);
        /*****************************************/
            listener(&mut current_mod);
        /*****************************************/
        if is_key_down(KeyCode::Space) {
            clean_screen(&mut strokes,&mut rectangles,&mut circles,&mut lines);
        }
        /*****************************************/
            navbar(&pen);
        /*****************************************/
        match current_mod{
            Mod::Pen=>pen_mod(tx.clone() ,&mut strokes),
            Mod::Rectangle=>get_2_point_mod(&mut rectangles),
            Mod::Circle=>get_2_point_mod(&mut circles),
            Mod::Line=>get_2_point_mod(&mut lines),
        }
        /*****************************************/
            draw_strokes(&strokes);
            draw_rectangles(&rectangles);
            draw_circles(&circles);
            draw_lines(&lines);
        /*****************************************/
        for stroke in remote_strokes.lock().unwrap().iter() {
            for i in 1..stroke.len() {
                let (x1, y1) = stroke[i-1];
                let (x2, y2) = stroke[i];
                draw_line(x1, y1, x2, y2, 2.0, RED);
            }
        }


            next_frame().await;
    }
}



fn draw_lines(lines: &Vec<Vec<(f32, f32)>>){
    for line in lines{
        if line.len()==2{
            draw_line(line[0].0,line[0].1,line[1].0,line[1].1,1.0,BLACK);
        }else if line.len()==1{
            let mouse_pos = mouse_position();
            draw_line(line[0].0,line[0].1,mouse_pos.0,mouse_pos.1,1.0,BLACK);
        }
    }
}

fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}

fn draw_circles(circles: &Vec<Vec<(f32, f32)>>){
    for circle in circles{
        if circle.len()==2{
            let radius=distance(circle[0],circle[1]);
            draw_circle(circle[0].0,circle[0].1,radius,BLACK);
        }else if circle.len()==1{
            let mouse_pos = mouse_position();
            let radius=distance(circle[0],mouse_pos);
            draw_circle(circle[0].0,circle[0].1,radius,BLACK);
        }
    }
}

fn get_2_point_mod(shapes: &mut Vec<Vec<(f32, f32)>>) {
    if is_mouse_button_released(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.1 > TOP {
            match shapes.last_mut() {
                Some(last_rect) if last_rect.len() == 1 => { last_rect.push(mouse_pos);}
                _ => shapes.push(vec![mouse_pos]),
                
            }
        }
    }
}

fn draw_rectangles(rectangles: &Vec<Vec<(f32, f32)>>){
    for rec in rectangles{
        if rec.len()<2{
           continue;
        }
        let (x1, y1) = rec[0];
        let (x2, y2) = rec[1];

        let x = x1.min(x2);
        let y = y1.min(y2);

        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();
        draw_rectangle(x, y, width, height, BLACK);
    }
}

fn clean_screen(strokes : &Arc<Mutex<Vec<Vec<(f32, f32)>>>>,
                rectangles: &mut Vec<Vec<(f32, f32)>>,
                circles: &mut Vec<Vec<(f32, f32)>>,
                lines: &mut Vec<Vec<(f32, f32)>>,
){
    // *strokes=vec![vec![]];
    strokes.lock().unwrap().clear();
    *rectangles=vec![vec![]];
    *circles=vec![vec![]];
    *lines=vec![vec![]];
}

fn pen_mod(tx : mpsc::UnboundedSender<Action> ,strokes : &mut Arc<Mutex<Vec<Vec<(f32, f32)>>>>){
    /*****************************************/
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.1>TOP{
            // strokes.last_mut().unwrap().push(mouse_pos);
            strokes.lock().unwrap().last_mut().unwrap().push(mouse_pos);
        }else{
            // strokes.push(vec![]);
            strokes.lock().unwrap().push(vec![]);
        }
    } else if !strokes.lock().unwrap().last().unwrap().is_empty() {
        let finished = strokes.lock().unwrap().last().unwrap().clone();
        tx.send(Action::Stroke(finished)).unwrap();
       strokes.lock().unwrap().push(vec![]);
    }
}

fn draw_strokes(strokes: &Arc<Mutex<Vec<Vec<(f32, f32)>>>>){
    for stroke in strokes.lock().unwrap().iter() {
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

    // Draw white rec button
    draw_rectangle(0.0,TOP, screen_width(), screen_height()-TOP, WHITE);

    // Draw circle button
    draw_circle(88.0,27.0,12.0,BLACK);

    // Draw line button
    draw_line(110.0,15.0,134.0,39.0,3.0,BLACK);
}

fn listener(current_mod: &mut Mod) {
    let (x, y) = mouse_position();

    // Ignore clicks below the navbar
    if y > TOP {
        return;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        if x >= 15.0 && x <= 39.0 && y >= 15.0 && y <= 39.0 {
            *current_mod = Mod::Pen;
        } else if x >= 44.0 && x <= 68.0 && y >= 15.0 && y <= 39.0 {
            *current_mod = Mod::Rectangle;
        } else if (x - 88.0).powi(2) + (y - 27.0).powi(2) <= 12.0f32.powi(2) {
            *current_mod = Mod::Circle;
        } else if x >= 110.0 && x <= 134.0 && y >= 15.0 && y <= 39.0 {
            *current_mod = Mod::Line;
        }
    }
}