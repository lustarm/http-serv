use crate::http;
use crate::parse::parse_req;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::router::Router;

pub struct Handler {
    writer: TcpStream,
    router: Router,
    // payload: http::HttpPayload,
}

const BUFFER_MAX: usize = 9000;

impl Handler {
    pub fn new(writer: TcpStream, router: Router) -> Self {
        Handler { writer, router }
    }

    pub async fn handle_get(&mut self, payload: http::HttpPayload) -> std::io::Result<()> {
        for (key, value) in self.router.clone().get_routes().into_iter() {
            if payload.clone().get_path() != key {
                self.router.clone().not_found(&mut self.writer).await?;
            }
            value(&mut self.writer).await?;
        }
        Ok(())
    }

    pub async fn handle_request(&mut self) -> std::io::Result<()> {
        let mut reader = [0; BUFFER_MAX];
        self.writer.read(&mut reader).await?;

        let payload = parse_req(&reader);

        match payload.clone().get_type() {
            http::HttpType::_NONE => {}
            http::HttpType::GET => {
                self.handle_get(payload).await?;
            }
            http::HttpType::_POST => {}
        }
        Ok(())
    }
}
