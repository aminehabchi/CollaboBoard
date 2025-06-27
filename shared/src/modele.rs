use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Mod {
    Pen,
    Rectangle,
    RectangleLines,
    Circle,
    CircleLines,
    Line,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Type {
    Release,
    Clean,
    Click,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Data {
    pub mode: Mod,
    pub draw_mode: Type,
    pub last: (f32, f32),
}

#[derive(Clone)]
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
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.rectangles.clear();
        self.rectangles_lines.clear();
        self.circles.clear();
        self.circles_lines.clear();
        self.lines.clear();
    }
}
