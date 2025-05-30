use serde::{Serialize, Deserialize};
use bincode;


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

/***********************************/

pub fn draw_all_shapes(shapes :&Shapes){
    draw_strokes(&shapes.strokes);
    draw_lines(&shapes.lines);
    draw_rectangles(&shapes.rectangles);
    draw_rectangles_lines(&shapes.rectangles_lines);
    draw_circles(&shapes.circles);
    draw_circles_lines(&shapes.circles_lines);
}

/***********************************/

pub fn clean_screen(shapes :&mut Shapes){
    shapes.strokes=vec![vec![]];
    shapes.rectangles=vec![vec![]];
    shapes.rectangles_lines=vec![vec![]];
    shapes.circles=vec![vec![]];
    shapes.circles_lines=vec![vec![]];
    shapes.lines=vec![vec![]];
}

/***********************************/

use macroquad::prelude::*;

pub const TOP: f32 = 54.0;

pub fn draw_strokes(strokes: &Vec<Vec<(f32, f32)>>){
    for stroke in strokes {
        for i in 1..stroke.len() {
            let (x1, y1) = stroke[i - 1];
            let (x2, y2) = stroke[i];
            draw_line(x1, y1, x2, y2, 2.0, BLACK);
        }
    }
}


pub fn draw_lines(lines: &Vec<Vec<(f32, f32)>>) {
    for line in lines {
        if line.len() == 2 {
            draw_line(line[0].0, line[0].1, line[1].0, line[1].1, 1.0, BLACK);
        }
    }
}

fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}


pub fn draw_circles(circles: &Vec<Vec<(f32, f32)>>){
    for circle in circles{
        if circle.len()==2{
            let radius=distance(circle[0],circle[1]);
            if circle[0].1-radius>TOP{
            draw_circle(circle[0].0,circle[0].1,radius,BLACK);
            }
        }
    }
}

pub fn draw_circles_lines(circles_lines: &Vec<Vec<(f32, f32)>>){
    for circle in circles_lines{
        if circle.len()==2{
            let radius=distance(circle[0],circle[1]);
            if circle[0].1-radius>TOP{
                draw_circle_lines(circle[0].0,circle[0].1,radius,1.0,BLACK);
            }
        }
    }
}


pub fn draw_rectangles(rectangles: &Vec<Vec<(f32, f32)>>){
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

pub fn draw_rectangles_lines(rectangles: &Vec<Vec<(f32, f32)>>) {
    for rec in rectangles {
        if rec.len() < 2 {
            continue;
        }

        let (x1, y1) = rec[0];
        let (x2, y2) = rec[1];

        let x = x1.min(x2);
        let y = y1.min(y2);
        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();

        draw_line(x, y, x + width, y, 1.0, BLACK);           
        draw_line(x + width, y, x + width, y + height, 1.0, BLACK); 
        draw_line(x + width, y + height, x, y + height, 1.0, BLACK); 
        draw_line(x, y + height, x, y, 1.0, BLACK);          
    }
}

