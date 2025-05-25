use tokio::{
    net::{TcpListener, TcpStream},
    io::AsyncWriteExt,
    sync::Mutex as AsyncMutex,
    time::{self, Duration},
};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use bincode;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shapes {
    pub strokes: Vec<Vec<(f32, f32)>>,
    pub rectangles: Vec<Vec<(f32, f32)>>,
    pub circles: Vec<Vec<(f32, f32)>>,
    pub lines: Vec<Vec<(f32, f32)>>,
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            strokes: vec![],
            rectangles: vec![],
            circles: vec![],
            lines: vec![],
        }
    }
}

type SharedClient = Arc<AsyncMutex<TcpStream>>;

pub async fn tcp_server(shared_shapes: Arc<Mutex<Shapes>>) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    let clients = Arc::new(AsyncMutex::new(Vec::<SharedClient>::new()));
    let clients_clone = Arc::clone(&clients);
    let shapes_clone = Arc::clone(&shared_shapes);

    // Spawn the broadcaster
    tokio::spawn(update_client_window(clients_clone, shapes_clone));

    // Accept new clients
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);
        clients.lock().await.push(Arc::new(AsyncMutex::new(socket)));
    }
}

async fn update_client_window(
    clients_clone: Arc<AsyncMutex<Vec<SharedClient>>>,
    shapes_clone: Arc<Mutex<Shapes>>,
) {
    let mut interval = time::interval(Duration::from_millis(16));
    loop {
        interval.tick().await;

        // Clone shapes safely
        let shapes = {
            let lock = shapes_clone.lock().unwrap();
            lock.clone()
        };

        // Serialize shapes
        let data = match bincode::serialize(&shapes) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Serialization error: {:?}", e);
                continue;
            }
        };
        let len = (data.len() as u32).to_be_bytes();

        // Lock and iterate clients
        let mut clients_guard = clients_clone.lock().await;
        let mut i = 0;
        while i < clients_guard.len() {
            let client = Arc::clone(&clients_guard[i]);
            let len = len.clone();
            let data = data.clone();

            let result = tokio::spawn(async move {
                let mut client = client.lock().await;
                client.write_all(&len).await?;
                client.write_all(&data).await?;
                Ok::<(), std::io::Error>(())
            }).await;

            match result {
                Ok(Ok(())) => i += 1,
                _ => {
                    clients_guard.remove(i);
                    eprintln!("Dropped a client due to write failure.");
                }
            }
        }
    }
}
