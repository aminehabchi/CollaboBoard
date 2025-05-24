use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub enum Action {
    Stroke(Vec<(f32,f32)>),
}