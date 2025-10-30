pub mod app_config;
pub mod status_code;
pub async fn init_config()
{
    app_config::app_init().await;
    status_code::init_status_code().await;
}