#[tokio::main]
async fn main() {
    // Create an asynchronous TCP listener bound to 127.0.0.1:8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address 127.0.0.1:8080");
    
    // Confirm the server is running
    println!("Server listening on 127.0.0.1:8080");
    
    // Main loop to continuously accept incoming TCP connections
    while let Ok((_stream, addr)) = listener.accept().await {
        println!("Accepted connection from: {}", addr);
        // For now, we just accept the connection
        // In future steps, we'll handle the connection in a separate task
    }
}
