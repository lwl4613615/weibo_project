use axum::{Json, response::{IntoResponse, Response}};

use crate::service::user::create_user_service;


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
pub async fn create_user_handler(Json(user):Json<ReqCreateUser>) -> Response {

    if let Ok(uid) = create_user_service(user).await {
        Json(serde_json::json!({
            "code":200,
            "msg":"用户创建成功",
            "data":{
                "uid":uid
            }
        })).into_response()
    } else {
        Json(serde_json::json!({
            "code":500,
            "msg":"用户创建失败"
        })).into_response()
    }
} 