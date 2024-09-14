use futures::FutureExt;
use log::info;
use std::sync::Arc;
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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Create routes
    let mut router = router::Router::new();
    router.add_route("/", Arc::new(Box::new(|input| (test(input).boxed()))));

    info!("Created route /");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("listening on port 8080");

    let service = http::HttpService::new(listener, router);
    service.listen_and_serve().await?;

    Ok(())
}
