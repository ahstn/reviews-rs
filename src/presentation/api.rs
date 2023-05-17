use axum::{handler::get, Router};
use hyper::Server;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn start_api_server(addr: SocketAddr) {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let server = Server::from_tcp(TcpListener::bind(addr).await.unwrap())
        .unwrap()
        .http(app)
        .run();
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}
