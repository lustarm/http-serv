use log::info;
use tokio::net::TcpListener;

mod handle;
mod http;
mod parse;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Create routes
    let mut router = handle::Router::new();
    router.add_route("/".to_string(), "hello".to_string());
    info!("Created route /");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("listening on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        let router = router.clone();

        tokio::spawn(async move {
            /*
                GET / HTTP/1.1
                Host: localhost:8080
                User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0
                Accept
            */

            handle::handle_req(socket, router).await;
        });
    }
}
