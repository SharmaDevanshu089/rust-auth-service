use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //YE LOCALHOST PE 3000 PORT PE SOCKET BANAYEGA
    let websocket_address = SocketAddr::from(([0, 0, 0, 0], 3000));
}
