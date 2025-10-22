use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router
};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo{
    id: u32,
    title: String,
    completed: bool,
    created_at: String,
}

#[derive(Deserialize)]
struct CreateTodo{
    title: String
}

#[derive(Deserialize)]
struct UpdateTodo{
    title: Option<String>,
    completed: Option<bool>
}

// Create a shared state
type AppState = Arc<Mutex<HashMap<u32, Todo>>>;

//Create Handlers
// List all existant tasks
async fn list_todos (State(state): State<AppState>
    ) -> Json<Vec<Todo>>
    {
    let todos = state.lock().unwrap();
    Json(todos.values()
              .cloned()
              .collect())
}

// Get a specific tasks
async fn get_todo(Path(id): Path<u32>,
    State(state): State<AppState>
    )-> Result<Json<Todo>, StatusCode>
    {
    let todos = state.lock().unwrap();
    todos.get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)

}

//create tasks
async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>)
    ->(StatusCode, Json<Todo>){
        let mut todos = state.lock().unwrap();

        let id = todos.keys().max().unwrap_or(&0) +1;
        let todo = Todo{
            id: id as u32,
            title: payload.title,
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339()
        };
        todos.insert(id, todo.clone());

        (StatusCode::CREATED, Json(todo))
    }
async  fn update_todo(
        State(state): State<AppState>,
        Path(id): Path<u32>, 
        Json(payload): Json<UpdateTodo>,
    )-> Result< Json<Todo>, StatusCode> {
        let mut todos = state.lock().unwrap();

        let todo = todos.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
        
        if let Some(title) = payload.title{
            todo.title = title;
        }
        if let Some(completed) = payload.completed{
            todo.completed = completed;
        }

        Ok(Json(todo.clone()))
}
    
//delete task
async fn delete_todo(
        State(state): State<AppState>,
        Path(id): Path<u32>)
        -> StatusCode
        {
        let mut todos
                = state.lock()
                       .unwrap();
        if todos.remove(&id).is_some(){
            StatusCode::NO_CONTENT
        }
        else{
            StatusCode::NOT_FOUND
        }
} 


#[tokio::main]
async fn main(){

    let state = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
            .route("/api/todos",
                get(list_todos))
            .route("/api/todos", 
                post(create_todo)
            )
            .route("/api/todos/{id}",
                get(get_todo)
                .delete(delete_todo)
                .put(update_todo)
            ).with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap();
    println!("🚀 API Todo démarrée sur http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}  