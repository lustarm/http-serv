use futures::future::BoxFuture;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::http;

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

    pub fn get_routes(self) -> HashMap<String, RouterFunctionWrapper> {
        self.routes
    }
}
