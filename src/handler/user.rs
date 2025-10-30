use axum::{Json, response::Response};

use crate::service::user::create_user_service;
use crate::utils::result::response_json;

pub async fn  home_handler() -> String {
    "Welcome to the Home Page".to_string()
}
#[derive(Debug,serde::Deserialize)]
pub struct ReqCreateUser {
    pub phone:String,
    pub password:String,
    pub nickname:Option<String>,
    pub avatar:Option<String>,
    pub gender:Option<i16>,
    pub birthday:Option<i32>
}
#[derive(serde::Serialize)]
struct RespUserId {
    pub uid:u32
}
pub async fn create_user_handler(Json(user):Json<ReqCreateUser>) -> Response {

    if let Ok(uid) = create_user_service(user).await {
        response_json(0,RespUserId{
            uid
        }).await
   
    } else {
        response_json(1,()).await
   
    }

} 