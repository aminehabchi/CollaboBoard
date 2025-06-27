use std::sync::{ Arc };
use tokio::{
    net::{ TcpStream },
    sync::Mutex as AsyncMutex,
};

mod server;
use server::*;

use shared::*;

mod window;
use window::*;

#[macroquad::main("Server White Board")]
async fn main() {
    let clients = Arc::new(AsyncMutex::new(Vec::<TcpStream>::new()));
    let shapes = Arc::new(AsyncMutex::new(Shapes::new()));

    let clients_clone = Arc::clone(&clients);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(async move {
        tcp_server(clients_clone).await.unwrap();
    });

    update_window(shapes, clients).await.expect("REASON");
}
