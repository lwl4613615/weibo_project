pub mod user;
use axum::{Router, response::Redirect, routing::get};
use user::user_routers;

pub async fn start_route() {
    // build our application with a single route
    let user_router = user_routers();
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/user/home") }))
        .nest("/user", user_router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
