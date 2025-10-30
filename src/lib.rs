pub mod config;
pub mod entities;
pub mod handler;
pub mod middleware;
pub mod utils;
pub mod model;
pub mod service;
pub mod router;

pub async fn run() {
   
   config::init_config().await;
   model::db_conn_init().await;
   println!("App Configs: {:?}", config::app_config::APP_CONFIGS.get().unwrap());
   self::router::start_route().await;
}