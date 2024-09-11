use crate::parse::parse_req;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_req(mut socket: TcpStream) {
    let mut buffer: [u8; 128] = [0; 128];
    socket.read(&mut buffer).await.unwrap();

    // Parse
    parse_req(input);
}
