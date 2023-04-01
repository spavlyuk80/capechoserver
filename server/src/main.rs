use std::io::Error;
use tokio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;





async fn handle_msg(stream: TcpStream) -> Result<(), Error> {
    let mut buf_stream = BufStream::new(stream);

    loop {
        let mut line = String::new();
        let n = buf_stream.read_line(&mut line).await?;

        if n == 0 {
            // End of stream reached, connection closed by the client
            break;
        }

        if line.ends_with("\r\n") {
            println!("Received message: {}", line);
        }

        buf_stream.write_all(line.to_uppercase().as_bytes()).await?;
        buf_stream.flush().await?;
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8010").await.expect("Could not bind to the port");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = handle_msg(stream).await {
                println!("Error: {}", e);
            }
        }).await.unwrap();

    }
}