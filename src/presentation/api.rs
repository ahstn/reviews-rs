use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn start_api_server(addr: String) {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    axum::serve(listener, app).await.unwrap();
}
