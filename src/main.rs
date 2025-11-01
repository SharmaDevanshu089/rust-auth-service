use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //YE LOCALHOST PE 3000 PORT PE SOCKET BANAYEGA
    let websocket_address = SocketAddr::from(([0, 0, 0, 0], 3000));

    // ABHI EK TEMPORARY ROUTER BANA RAHA HU TEST KE LIYE BAAD ME ACCHE SE LIKH DUNGA
    // YE ROUTER ABHI KEVAL HELLO VALE KO CALL KAR RAH H
    let axium_router = Router::new().route("/", get(return_hello()));

    // AB ISS PORT KO BIND KARUNGA, ABHI ERROR KO UNWRAP KAR RAHA HU BAAD ME LOG KAURNGA

    // ABHI DOCS READ KIE NEW AXUM ME SERVER KI JAGAH SERVE USE HOTA HAI AUR AB TOKIO KA TCP LISNER USE HOGA

    // CREATING A TCP LISNER , ISS ERROR KO FUTURE MAI HANDLE KARUNGA

    let new_tcp_lisner_for_serve = TcpListener::bind(websocket_address).await.unwrap();

    axum::serve(new_tcp_lisner_for_serve, axium_router.into_make_service())
        .await
        .unwrap();
    // axum::Server::bind(&websocket_address)
    //     .serve(axium_router.into_make_service())
    //     .await
    //     .unwrap();
}

// YE KEVAL TESTING KE LIYE H , KUCH KAAM KA NAHI HAI SIRF HELLO RETURN KARTA HAI
// MUJHE TEST KARNA THA ISS LIYE PUT KIA
async fn return_hello() -> &'static str {
    let hello = "Hello Via Internet";
    return hello;
}
