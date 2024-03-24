use crate::handlers::{create_user, get_applicants, update_user};
use axum::{
    routing::{get, patch, post},
    Router,
};

pub async fn create_routes() -> Router {
    Router::new()
        .route("/applicants", get(get_applicants)) 
        .route("/sign_up", post(create_user))
        .route("/update/:id", patch(update_user))
}
