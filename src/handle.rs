use crate::http;
use crate::parse::parse_req;
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::router::Router;

pub async fn handle_get(
    mut writer: TcpStream,
    payload: http::HttpPayload,
    router: Router,
) -> std::io::Result<()> {
    for (key, value) in router.clone().get_routes().into_iter() {
        if payload.clone().get_path() != key {
            router.clone().not_found(&mut writer).await?;
        }
        value(&mut writer).await?;
    }
    Ok(())
}

pub async fn handle_req(mut socket: TcpStream, router: Router) -> std::io::Result<()> {
    let mut buffer: [u8; 128] = [0; 128];
    socket.read(&mut buffer).await.unwrap();

    // Parse
    let http_payload: http::HttpPayload = parse_req(&buffer);

    match http_payload.clone().get_type() {
        http::HttpType::NONE => {
            info!("Invalid request type");
            let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            if let Err(e) = socket.write_all(error_response.as_bytes()).await {
                info!("Failed to send error response: {}", e);
            }
        }
        http::HttpType::GET => {
            handle_get(socket, http_payload, router).await?;
        }
        http::HttpType::_POST => {}
    }

    Ok(())
}
