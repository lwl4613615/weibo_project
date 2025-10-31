pub mod config;
pub mod entities;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;
pub mod utils;

pub async fn run() {
    config::init_config().await;
    utils::jwt::init_jwt_secret().await;
    model::db_conn_init().await;
    //println!("App Configs: {:?}", config::app_config::APP_CONFIGS.get().unwrap());
    self::router::start_route().await;
}
