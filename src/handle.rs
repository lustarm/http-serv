use crate::http;
use crate::parse::parse_req;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// Simple router for testing
#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, key: String, payload: String) {
        self.routes.insert(key, payload);
    }

    pub fn get_route(&self, key: &str) -> Option<&String> {
        self.routes.get(key)
    }

    pub fn get_routes(self) -> HashMap<String, String> {
        self.routes
    }
}

pub async fn handle_get(mut socket: TcpStream, payload: http::HttpPayload, router: Router) {
    for (key, value) in router.routes.into_iter() {
        match payload.clone().get_path() {
            key => {
                socket
                    .write_all(
                        http::HttpResponse::new("HTTP/1.1", http::HttpStatus::Ok, &value)
                            .to_string()
                            .as_bytes(),
                    )
                    .await
                    .unwrap();
            }
            _ => {
                socket
                    .write_all(
                        http::HttpResponse::new("HTTP/1.1", http::HttpStatus::NotFound, "404")
                            .to_string()
                            .as_bytes(),
                    )
                    .await
                    .unwrap();
            }
        }
    }
}

pub async fn handle_req(mut socket: TcpStream, router: Router) {
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
            return;
        }
        http::HttpType::GET => {
            handle_get(socket, http_payload, router).await;
        }
        http::HttpType::_POST => {}
    }
}
