use crate::handlers::{create_user, get_applicants};
use axum::{
    routing::{get, post},
    Router,
};

pub async fn create_routes() -> Router {
    Router::new()
        .route("/applicants", get(get_applicants)) 
        .route("/api/sign_up", post(create_user))
}
