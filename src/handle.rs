use crate::http;
use crate::parse::parse_req;
use futures::future::BoxFuture;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

type RouterFunction = dyn Fn(&mut TcpStream) -> BoxFuture<std::io::Result<()>> + Send + Sync;

type RouterFunctionWrapper = Arc<Box<RouterFunction>>;

// Simple router for testing
#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, RouterFunctionWrapper>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, key: &str, payload: RouterFunctionWrapper) {
        self.routes.insert(key.to_string(), payload);
    }

    pub async fn not_found(self, writer: &mut TcpStream) -> std::io::Result<()> {
        writer
            .write_all(
                http::HttpResponse::new(
                    "HTTP/1.1",
                    http::HttpStatus::NotFound,
                    "404 page not found",
                )
                .to_string()
                .as_bytes(),
            )
            .await?;

        Ok(())
    }

    /*
    pub fn get_route(&self, key: &str) -> Option<&String> {
        self.routes.get(key)
    }

    pub fn get_routes(self) -> HashMap<String, String> {
        self.routes
    }
    */
}

pub async fn handle_get(
    mut writer: TcpStream,
    payload: http::HttpPayload,
    router: Router,
) -> std::io::Result<()> {
    for (key, value) in router.routes.clone().into_iter() {
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
