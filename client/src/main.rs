use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;
use tokio::time;
use common::add_end_of_msg;

async fn connect() -> TcpStream {
    loop {
        match TcpStream::connect("127.0.0.1:8010").await {
            Ok(stream) => return stream,
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stream = connect().await;
    let mut stdin_reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    let mut buf_stream = BufStream::new(stream);

    loop {
        // cli reader
        println!("Enter a message to send to the server: ");
        tokio::io::stdout().flush().await?;
        let mut input = match stdin_reader.next_line().await {
            Ok(Some(line)) => line,
            _ => break
        };
        let msg_with_eof = add_end_of_msg(&mut input).await?;

        // write to stream
        buf_stream.write_all(msg_with_eof.as_bytes()).await.expect("TODO: panic message");
        buf_stream.flush().await?;

        // read stream
        let mut line = String::new();
        match buf_stream.read_line(&mut line).await {
            Ok(n) if n == 0 => {
                println!("Connection closed by server");
                break;
            }
            Ok(_) => {
                if line.ends_with("\r\n") {
                    println!("Received message from server: {}", line.trim());
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }

        println!("You entered: {}\n", msg_with_eof);
    }
    Ok(())
}