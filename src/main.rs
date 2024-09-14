use futures::FutureExt;
use log::info;
use std::{io, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

mod handle;
mod http;
mod parse;
mod router;

// Test function
// W = writer = socket
async fn test(w: &mut TcpStream) -> std::io::Result<()> {
    info!("this is a test!");
    w.write_all(
        http::HttpResponse::new("HTTP/1.1", http::HttpStatus::Ok, "test")
            .to_string()
            .as_bytes(),
    )
    .await
    .unwrap();
    Ok(())
}

struct HttpService {
    listener: TcpListener,
    router: router::Router,
}

impl HttpService {
    pub fn new(listener: TcpListener, router: router::Router) -> Self {
        HttpService { listener, router }
    }

    pub async fn listen_and_serve(self) -> io::Result<()> {
        loop {
            let (writer, _) = self.listener.accept().await?;
            let router = self.router.clone();

            tokio::spawn(async move {
                /*
                    GET / HTTP/1.1
                    Host: localhost:8080
                    User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0
                    Accept
                */

                handle::handle_req(writer, router)
                    .await
                    .unwrap_or_else(|_| {
                        info!("Failed to handle client!");
                    });
            });
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Create routes
    let mut router = router::Router::new();
    // Instead of doing just the body
    // add some functionallity with Fn()
    router.add_route("/", Arc::new(Box::new(|input| (test(input).boxed()))));
    info!("Created route /");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("listening on port 8080");

    let service = HttpService::new(listener, router);
    service.listen_and_serve().await?;

    Ok(())
}
