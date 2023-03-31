use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    loop {
        print!("Enter a message to send to the server: ");
        tokio::io::stdout().flush().await?;

        let input = match reader.next_line().await {
            Ok(Some(line)) => line,
            _ => break,
        };

        let mut stream = TcpStream::connect("127.0.0.1:8009").await?;
        stream.write_all(input.as_bytes()).await?;
        stream.flush().await?;

        let mut buf = vec![0; 512];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[0..n]);
        println!("Server response: {}", response);
    }

    Ok(())
}