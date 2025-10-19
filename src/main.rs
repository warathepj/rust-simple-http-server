use std::error::Error;
use std::str::from_utf8;
use tokio::fs; // Import tokio's fs module
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn Error>> {
    println!("Handling connection from: {}", addr);
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).await?;

    // If no bytes were read, the connection might have closed
    if bytes_read == 0 {
        return Ok(());
    }

    let request = from_utf8(&buffer[..bytes_read])?;

    let mut lines = request.lines();
    if let Some(first_line) = lines.next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let _method = parts[0]; // Method is not used for routing in this example, but extracted
            let path = parts[1];
            println!("Request Method: {}, Path: {}", _method, path);

            let (status_line, contents, content_type) = match path {
                "/" => {
                    // Try to read index.html from the static directory
                    match fs::read_to_string("C:\\Users\\warat\\Documents\\project\\rust\\http-server\\simple_http_server\\static\\index.html").await {
                        Ok(file_contents) => ("HTTP/1.1 200 OK", file_contents, "text/html"),
                        Err(e) => {
                            eprintln!("Failed to read index.html: {}", e);
                            ("HTTP/1.1 500 Internal Server Error", "500 Internal Server Error".to_string(), "text/plain")
                        }
                    }
                }
                "/sleep" => {
                    sleep(Duration::from_secs(5)).await;
                    (
                        "HTTP/1.1 200 OK",
                        "Slept for 5 seconds!".to_string(),
                        "text/plain",
                    )
                }
                _ => (
                    "HTTP/1.1 404 NOT FOUND",
                    "404 Not Found".to_string(),
                    "text/plain",
                ),
            };

            let response = format!(
                "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                content_type,
                contents.len(),
                contents
            );

            stream.write_all(response.as_bytes()).await?;
        } else {
            eprintln!("Malformed request line (not enough parts): {}", first_line);
            let bad_request_response =
                "HTTP/1.1 400 Bad Request\r\nContent-Length: 15\r\n\r\nBad Request\r\n";
            stream.write_all(bad_request_response.as_bytes()).await?;
        }
    } else {
        eprintln!("Received empty or malformed request (no first line).");
        let bad_request_response =
            "HTTP/1.1 400 Bad Request\r\nContent-Length: 15\r\n\r\nBad Request\r\n";
        stream.write_all(bad_request_response.as_bytes()).await?;
    }

    Ok(())
}
