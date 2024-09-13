use crate::http;
use crate::parse::parse_req;
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_get(mut socket: TcpStream, payload: http::HttpPayload) {
    // Hardcode paths
    match payload.get_path().as_str() {
        "/" => {
            socket
                .write_all(
                    http::HttpResponse::new("HTTP/1.1", http::HttpStatus::Ok, "hello, world")
                        .to_string()
                        .as_bytes(),
                )
                .await
                .unwrap();
        }
        _ => {
            socket
                .write_all(
                    http::HttpResponse::new("HTTP/1.1", http::HttpStatus::Ok, "404")
                        .to_string()
                        .as_bytes(),
                )
                .await
                .unwrap();
        }
    }
}

pub async fn handle_req(mut socket: TcpStream) {
    let mut buffer: [u8; 128] = [0; 128];
    socket.read(&mut buffer).await.unwrap();

    // Parse
    let http_payload: http::HttpPayload = parse_req(&buffer);
    // So fucking ugly i can't lie
    let http_payload_clone = http_payload.clone();

    match http_payload.get_type() {
        http::HttpType::NONE => {
            info!("Invalid request type");
            // Optionally, you might want to send a 400 Bad Request response here
            let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            if let Err(e) = socket.write_all(error_response.as_bytes()).await {
                info!("Failed to send error response: {}", e);
            }
            return;
        }
        http::HttpType::GET => {
            handle_get(socket, http_payload_clone).await;
        }
        http::HttpType::_POST => {
            handle_get(socket, http_payload_clone).await;
        }
    }
}
