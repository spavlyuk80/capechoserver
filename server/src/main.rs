use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


async fn handle_msg(mut stream: TcpStream) -> Result<(), String> {

    let mut buf = Vec::new();
    let n = stream.read_to_end(&mut buf).await.unwrap();
    println!("Read {} bytes from client", n);
    let message = String::from_utf8_lossy(&buf);
    println!("Buffer contents: {}", message);
    stream.write_all("ok".as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8009").await.expect("Could not bind to the port");

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            if let Err(e) = handle_msg(stream).await {
                println!("Error: {}", e);
            }
        }).await.unwrap();

    }
}