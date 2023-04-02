use common::Message;
use std::io::Error;
use std::time::Duration;
use tokio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};
use tokio::{select, time};

// creates a one connection server
// TODO handle multiple connections
async fn create_server() -> TcpListener {
    loop {
        match TcpListener::bind("127.0.0.1:8000").await {
            Ok(listener) => return listener,
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn handle_msg(mut stream: BufStream<TcpStream>) -> Result<(), Error> {
    loop {
        let mut line = String::new();

        // check what is in the buffer
        match stream.read_line(&mut line).await {
            Ok(n) if n == 0 => {
                println!("Connection closed by server");
                break;
            }
            Ok(_) => {
                let msg = Message::new(line);
                if Message::has_end_of_msg(&msg) {
                    println!("Received message: {}", &msg.value);
                    let formatted_msg = Message::to_upper_case(&msg);
                    println!("Formatted message: {}", &formatted_msg);
                    stream.write_all(formatted_msg.as_bytes()).await?;
                    stream.flush().await?;
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = create_server().await;

    loop {
        // just a quick workaround to handle multiple connections at once
        select! {
            // wait for incoming connection
            Ok((stream, _)) = listener.accept() => {
                let buf_stream = BufStream::new(stream);

                // spawn a new task to handle the client
                tokio::spawn(async move {
                    if let Err(e) = handle_msg(buf_stream).await {
                        println!("Error: {}", e);
                    }
                });
            }
            _ = time::sleep(Duration::from_millis(100)) => {
                // do nothing, just wait
            }
        }
    }
}
