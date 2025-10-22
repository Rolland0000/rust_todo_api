use axum::{
    http::StatusCode, 
    routing::get,
    Router
};


async fn hello() -> &'static str{
    "Hello World!"
}

async fn about()-> &'static str{
    "About my Axum server"
}

async fn health() -> (StatusCode, &'static str){
    (StatusCode::OK, "Servive is healthy")
}

async fn not_found() -> (StatusCode, &'static str){
    (StatusCode::NOT_FOUND, "Page not founded")
}

#[tokio::main]
async fn main() {

    // Construction of the rooter
    let app = Router::new()
        .route("/", get(hello))
        .route("/about", get(about))
        .route("/health", get(health))
        .fallback(not_found);

    // Starting server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!(" The server started on http://127.0.0.1:8080");
    axum::serve(listener,app)
        .await
        .unwrap();

}