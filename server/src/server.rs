use tokio::{ net::{ TcpListener, TcpStream }, sync::Mutex as AsyncMutex };

use anyhow::Result;

use std::sync::{ Arc };

pub async fn tcp_server(clients: Arc<AsyncMutex<Vec<TcpStream>>>) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    // Accept new clients
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);

        clients.lock().await.push(socket);
    }
}
