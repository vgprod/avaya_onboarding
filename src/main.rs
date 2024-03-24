mod models;
mod routes;
mod handlers;
mod storage;

use tokio::net::TcpListener;
use crate::routes::create_routes;



#[tokio::main]
async fn main() {

    let app = create_routes().await;

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8100").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
