use macroquad::prelude::*;
use tokio::{runtime::Runtime, net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use bincode;
use std::sync::{Arc, Mutex};

mod server;
use server::*;


use shared::*;

mod window;
use window::*;

#[macroquad::main("Server White Board")]
async fn main() {
    let mut shapes = Arc::new(Mutex::new(Shapes::new()));
    let shapes_clone = Arc::clone(&shapes);

    // Create Tokio runtime manually
    let rt = Runtime::new().unwrap();
    rt.spawn(tcp_server(shapes_clone));

 
     update_window(&mut shapes).await.expect("REASON") 
        
}