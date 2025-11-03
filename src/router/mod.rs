pub mod user;
pub mod follower;
use axum::{Router,  response::Redirect, routing::get};
use user::user_routers;



pub async fn start_route() {
    // 公开路由（不需要 token）
    let user_router = user_routers();
    
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/user/home") }))
        .nest("/user", user_router)
        .nest("/follower", follower::follower_routers());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
