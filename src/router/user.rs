use axum::{
    Router,  routing::{get, post}
};

use crate::{handler::user::{create_user_handler, home_handler, login_user_handler, profile_handler}};

pub fn user_routers() -> Router {
    Router::new()
        .route("/home", get(home_handler))
        .route("/users", post(create_user_handler))
        .route("/login", post(login_user_handler))        
        .route("/profile", get(profile_handler))
}
