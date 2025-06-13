use tokio::{
    net::{ TcpListener, TcpStream },
    io::AsyncWriteExt,
    sync::Mutex as AsyncMutex,
};

use anyhow::Result;

use bincode;
use std::sync::{ Arc, Mutex };




pub async fn tcp_server(
    clients: Arc<AsyncMutex<Vec<TcpStream>>>,
) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    let clients_clone = Arc::clone(&clients);


    // Accept new clients
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);

       
        clients.lock().await.push(socket);
    }
}
