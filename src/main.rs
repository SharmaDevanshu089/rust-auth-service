use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //YE LOCALHOST PE 3000 PORT PE SOCKET BANAYEGA
    let websocket_address = SocketAddr::from(([0, 0, 0, 0], 3000));
    // ABHI EK TEMPORARY ROUTER BANA RAHA HU TEST KE LIYE BAAD ME ACCHE SE LIKH DUNGA
    // YE ROUTER ABHI KEVAL HELLO VALE KO CALL KAR RAH H
    let axium_router = Router::new().route("/", get(return_hello()));
}

// YE KEVAL TESTING KE LIYE H , KUCH KAAM KA NAHI HAI SIRF HELLO RETURN KARTA HAI
// MUJHE TEST KARNA THA ISS LIYE PUT KIA
async fn return_hello() -> String {
    let hello = "Hello Via Internet".to_string();
    return hello;
}
