use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //YE LOCALHOST PE 3000 PORT PE SOCKET BANAYEGA
    let websocket_address = SocketAddr::from(([0, 0, 0, 0], 3000));
    // ABHI EK TEMPORARY ROUTER BANA RAHA HU TEST KE LIYE BAAD ME ACCHE SE LIKH DUNGA
    let axium_router = Router::new();
}
