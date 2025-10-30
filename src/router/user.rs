use axum::{Router, routing::{get, post}};

use crate::handler::user::{create_user_handler, home_handler};



pub fn user_routers()->Router{
    Router::new()
    .route("/home", get(home_handler))
    .route("/users",post(create_user_handler))
}