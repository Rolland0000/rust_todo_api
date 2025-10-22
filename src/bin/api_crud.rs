use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State, Json},
    Router,
    routing::{get, post, put, delete},
    http::StatusCode,
};
// use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct User{
    id: u8,
    name: String,
    phone: String,
    email: String,
    date: String,
}

#[derive(Clone)]
struct AppState{
     users: Arc<Mutex<Vec<User>>>,
}
async fn list_users(State(state): State<AppState>)-> (StatusCode, Json<Vec<User>>){

    let users = state.users.lock().unwrap();
    (StatusCode::OK, Json(users.clone().to_vec()))

}

async fn create_user(State(state): State<AppState>,Json(payload): Json<User>
    ) -> (StatusCode, Json<Vec<User>>){

    let users = state.users.lock().unwrap();
    users.to_vec().push(payload);

    (StatusCode::CREATED, Json(users.clone()))

}

async fn update_user(Path(id): Path<u8>, 
                     State(state): State<AppState>,
                     Json(payload): Json<User>
    ) -> Result<Json<User>, StatusCode>{
    
    let users = state.users.lock().unwrap();
    if let Some(user) = users.to_vec().iter_mut().find(|user| user.id == id){
        *user = payload;
        Ok(Json(user.clone()))
    }else{
        Err(StatusCode::NOT_FOUND)
    }
}
async fn get_user(Path(id): Path<u8>, State(state): State<AppState>
    ) -> Result<Json<User>, StatusCode>{
        let users = state.users.lock().unwrap();

        if let Some(user) = users.to_vec().iter().find(|user| user.id == id){
            Ok(Json(user.clone()))
        }
        else{
            Err(StatusCode::NOT_FOUND)
        }
}

async fn delete_user(Path(id): Path<u8>, State(state): State<AppState>
    ) -> StatusCode {
    let mut users = state.users.lock().unwrap();
        let len_befrore = users.len();
    users.retain(|user| user.id != id);
    let new_len = users.len();
    
    if len_befrore > new_len {
        
        StatusCode::NO_CONTENT
    }else {
        StatusCode::NOT_FOUND
    }
    
}

#[tokio::main]
async fn main(){
    let state = AppState{users: Arc::new(Mutex::new(Vec::new()))};

    let app = Router::new()
            .route("/api/users", get(list_users))
            .route("/api/users", post(create_user))
            .route("/api/users/{id}", get(get_user))
            .route("/api/users/{id}", put(update_user))
            .route("/api/users/{id}", delete(delete_user))
            .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
     axum::serve(listener, app).await.unwrap();

}

