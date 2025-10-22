use axum::{
    routing::get,
    Router,
    http::StatusCode};

async fn hello() -> &'static str{
    "Hello World !"
}

async fn about() -> &'static str{
    "A propos de mon serveur Axum"
}

async fn health() -> (StatusCode, &'static str){
    (StatusCode::OK, ("Service is healthy"))
}

async fn not_found() -> (StatusCode, &'static str){
    (StatusCode::NOT_FOUND, "Page non trouvée")
}

#[tokio::main]
async fn main(){

    //Construction du routeur
    let  app = Router::new()
            .route("/", get(hello))
            .route("/about", get(about))
            .route("/health", get(health))
            .fallback(not_found);
    
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
                 .await
                 .unwrap();
    println!("🚀 Serveur démarré sur http://127.0.0.1:8080");
    axum::serve(listener, app)
        .await
        .unwrap();
    }
