use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use validator::Validate;
use axum::extract::Path;
use crate::middleware::jwt::AuthClaims;
use crate::service::follower::follower_service;
use crate::service::follower::unfollower_service;
use crate::utils::result::response_json;
use crate::utils::warp::ErrorWrap;
#[derive(serde::Deserialize,validator::Validate)]
pub struct ReqCreateFollower {
    #[validate(range(min = 1,code="7"))]
    pub followee: u32,
}

pub async fn follow_handler( AuthClaims(claims):AuthClaims,Json(user):Json<ReqCreateFollower>) -> Response {
    // Implementation for following a user
    if let Err(e) = user.validate() {
       
        return ErrorWrap(e).into_response();
        
    }
    if claims.uid ==user.followee {
        return response_json(7, "不能关注自己");        
    }
    match follower_service(claims.uid, user.followee).await {
        Ok(_) => response_json(0, "关注成功"),
        Err(err) => response_json(1, err.to_string()),
    }
}
pub async fn unfollow_handler(AuthClaims(claims):AuthClaims,Path(followee_id):Path<u32>) -> Response {
    // Implementation for unfollowing a user
    if claims.uid==followee_id {
        return response_json(7, "不能取消关注自己");
    }
    match unfollower_service(claims.uid, followee_id).await {
        Ok(_) => response_json(0, "取消关注成功"),
        Err(err) => response_json(1, err.to_string()),
    }

}