use axum::{Json, response::Response};
use validator::Validate;
use axum::response::IntoResponse;
use crate::{service::user::create_user_service, utils::warp::{ErrorWrap}};
use crate::utils::result::response_json;


pub async fn  home_handler() -> String {
    "Welcome to the Home Page".to_string()
}
#[derive(Debug,serde::Deserialize,Validate)]
pub struct ReqCreateUser {
    #[validate(custom(function="crate::utils::validate::validate_phone",code="2"))]
    pub phone:String,
    #[validate(length(min = 6,max=20,code="3"))]
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
        return ErrorWrap(e).into_response();           
    }

    if let Ok(uid) = create_user_service(user).await {
        response_json(0,RespUserId{
            uid
        })
   
    } else {
        response_json(1,())
   
    }

} 