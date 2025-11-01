use axum::Router;
use axum::routing::get;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8;
use dotenvy;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
// use tracing::util::SubscriberExt;

#[derive(Clone)]
struct app_state {
    db_pool: bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

#[tokio::main]
async fn main() {
    // ENVRIMENT VARIABLES KO LOAD KARNA HAI DOTEVY SE SAVES TIME
    dotenvy::dotenv().ok();

    // TRACER KO INITIALISE KAR RAHA HU ENV KE BAAD TAKI LOG MAI ENV VAR DAAL SAKETE
    // ISSE ENV KE ERROR TRACE NAHI HO PAYENGE
    tracing_subscriber::fmt::init();

    //YE LOCALHOST PE 3000 PORT PE SOCKET BANAYEGA
    let websocket_address = SocketAddr::from(([0, 0, 0, 0], 3000));

    // ENVIRMENT SE DATABASE URL NIKALNA HAI
    let database_url = env::var("DATABASE_URL").unwrap();

    // PUTTING UPAR VALI VALUE IN STUCT OF MANAGER
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    let pool = bb8::Pool::builder().build(config).await.unwrap();

    // ABHI EK TEMPORARY ROUTER BANA RAHA HU TEST KE LIYE BAAD ME ACCHE SE LIKH DUNGA
    // YE ROUTER ABHI KEVAL HELLO VALE KO CALL KAR RAH H
    let axium_router = Router::new()
        .route("/", get(return_hello))
        .with_state(app_state);
    tracing::info!("Server Listening on {}", websocket_address.to_string());

    // DOCS READ KIE NEW AXUM ME SERVER KI JAGAH SERVE USE HOTA HAI AUR AB TOKIO KA TCP LISNER USE HOGA
    // CREATING A TCP LISNER , ISS UNWRAP KO FUTURE MAI HANDLE KARUNGA
    let new_tcp_lisner_for_serve = TcpListener::bind(websocket_address).await.unwrap();

    // AB ISS PORT KO BIND KARUNGA, ABHI ERROR KO UNWRAP KAR RAHA HU BAAD ME LOG KAURNGA
    axum::serve(new_tcp_lisner_for_serve, axium_router)
        .await
        .unwrap();

    /*axum::Server::bind(&websocket_address)
    .serve(axium_router.into_make_service())
    .await
    .unwrap();*/
}

// YE KEVAL TESTING KE LIYE H , KUCH KAAM KA NAHI HAI SIRF HELLO RETURN KARTA HAI
// MUJHE TEST KARNA THA ISS LIYE PUT KIA
async fn return_hello() -> &'static str {
    let hello = "Hello Via Internet";
    return hello;
}
