use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080");

    while let Ok((socket, _addr)) = listener.accept().await {
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    while let Ok(n) = socket.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        if let Ok(data) = String::from_utf8(buffer[..n].to_vec()) {
            println!("Received: {}", data);
            socket.write_all(b"Message received!").await.unwrap();
        }

        // Clear the buffer for the next iteration
        buffer = [0; 1024];
    }
}
