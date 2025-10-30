use axum::{Json, response::Response};
use validator::Validate;

use crate::service::user::create_user_service;
use crate::utils::result::response_json;


pub async fn  home_handler() -> String {
    "Welcome to the Home Page".to_string()
}
#[derive(Debug,serde::Deserialize,Validate)]
pub struct ReqCreateUser {
    #[validate(custom(function="crate::utils::validate::validate_phone",code="3"))]
    pub phone:String,
    #[validate(length(min = 6,max=20,code="4"))]
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

    if let Err(e)=user.validate(){
        for (_,v) in e.field_errors(){
            let code=v.get(0).unwrap().code.parse::<usize>().unwrap();
            return response_json(code,()).await;
        }            
    }

    if let Ok(uid) = create_user_service(user).await {
        response_json(0,RespUserId{
            uid
        }).await
   
    } else {
        response_json(1,()).await
   
    }

} 