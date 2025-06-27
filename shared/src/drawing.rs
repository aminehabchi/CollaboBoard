use macroquad::prelude::*;

use crate::modele::*;

pub fn navbar(pen: &Texture2D) {
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

/***********************************/

pub fn draw_all_shapes(shapes: &Shapes) {
    draw_strokes(&shapes.strokes);
    draw_lines(&shapes.lines);
    draw_rectangles(&shapes.rectangles);
    draw_rectangles_lines(&shapes.rectangles_lines);
    draw_circles(&shapes.circles);
    draw_circles_lines(&shapes.circles_lines);
}

/***********************************/

pub fn clean_screen(shapes: &mut Shapes) {
    shapes.strokes = vec![vec![]];
    shapes.rectangles = vec![vec![]];
    shapes.rectangles_lines = vec![vec![]];
    shapes.circles = vec![vec![]];
    shapes.circles_lines = vec![vec![]];
    shapes.lines = vec![vec![]];
}

/***********************************/

pub const TOP: f32 = 54.0;

pub fn draw_strokes(strokes: &Vec<Vec<(f32, f32)>>) {
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

pub fn draw_circles(circles: &Vec<Vec<(f32, f32)>>) {
    for circle in circles {
        if circle.len() == 2 {
            let radius = distance(circle[0], circle[1]);
            if circle[0].1 - radius > TOP {
                draw_circle(circle[0].0, circle[0].1, radius, BLACK);
            }
        }
    }
}

pub fn draw_circles_lines(circles_lines: &Vec<Vec<(f32, f32)>>) {
    for circle in circles_lines {
        if circle.len() == 2 {
            let radius = distance(circle[0], circle[1]);
            if circle[0].1 - radius > TOP {
                draw_circle_lines(circle[0].0, circle[0].1, radius, 1.0, BLACK);
            }
        }
    }
}

pub fn draw_rectangles(rectangles: &Vec<Vec<(f32, f32)>>) {
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
