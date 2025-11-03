use axum::Router;
use axum::routing::delete;
use axum::routing::post;
use crate::handler::follow::{follow_handler, unfollow_handler};
pub fn follower_routers() -> Router {
    Router::new()
         .route("/followers", post(follow_handler))
         .route("/followers/{uid}", delete(unfollow_handler))
        // .route("/followers", get(list_followers_handler))
        // .route("/following", get(list_following_handler))
}