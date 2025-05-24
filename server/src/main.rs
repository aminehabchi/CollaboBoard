use tokio::{
    net::{TcpListener},
    sync::broadcast,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, _) = broadcast::channel::<String>(100);

    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server running on 127.0.0.1:4000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut writer = BufWriter::new(writer);

            let mut line = String::new();

            // Task 1: Read from this client and broadcast
            let tx_clone = tx.clone();
            let read_task = tokio::spawn(async move {
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // connection closed
                        Ok(_) => {
                            if let Err(e) = tx_clone.send(line.clone()) {
                                eprintln!("Broadcast error: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break;
                        }
                    }
                }
            });

            // Task 2: Send broadcast messages to this client
            let write_task = tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    if writer.write_all(msg.as_bytes()).await.is_err() {
                        break;
                    }
                    if writer.flush().await.is_err() {
                        break;
                    }
                }
            });

            // Wait for either task to finish
            let _ = tokio::join!(read_task, write_task);
        });
    }
}
