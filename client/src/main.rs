use common::Message;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;
use tokio::time;

async fn connect() -> TcpStream {
    loop {
        match TcpStream::connect("127.0.0.1:8000").await {
            Ok(stream) => return stream,
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stream = connect().await;
    let mut stdin_reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    let mut buf_stream = BufStream::new(stream);

    loop {
        // cli reader
        println!("Enter a message to send to the server: ");
        tokio::io::stdout().flush().await.unwrap();
        let input = match stdin_reader.next_line().await {
            Ok(Some(line)) => line,
            _ => break,
        };

        let msg = Message::new(input);

        // write to stream
        buf_stream
            .write_all(Message::add_end_of_msg(&msg).as_bytes())
            .await
            .expect("TODO: panic message");
        buf_stream.flush().await.unwrap();

        // read stream
        let mut line = String::new();
        match buf_stream.read_line(&mut line).await {
            Ok(n) if n == 0 => {
                println!("Connection closed by server");
                break;
            }
            Ok(_) => {
                let received_msg = Message::new(line);
                if Message::has_end_of_msg(&received_msg) {
                    println!("\nYou entered: {}", Message::remove_end_of_msg(&msg));
                    println!(
                        "Received message from server: {}",
                        Message::remove_end_of_msg(&received_msg)
                    );
                    println!("-----------------------------------")
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
