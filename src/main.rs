use tokio::{net::TcpListener, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, sync::broadcast};

#[tokio::main]
async fn main() {
    println!("Winniio Cloud Server");
    let listener = TcpListener::bind("192.168.100.155:7000").await.unwrap();
    let (tx, _rx) = broadcast::channel(32);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Connection From: {}", addr);
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 { break };
                        println!("{}: {}", addr, line);
                        // Get size of the buffer and Print it
                        let size = tx.receiver_count();
                        println!("Buffer size: {}", size);
                        // Send the line to all other clients
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}