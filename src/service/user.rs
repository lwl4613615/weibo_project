use crate::{model::user::{User, UserInfo}, utils::jwt::Claims};
use crate::{
    handler::user::{ReqCreateUser, ReqLogin},
    model::user::{create_user_model, find_user},
    utils::crypto::md5,
};
use anyhow::{Ok, Result};

pub async fn create_user_service(ruser: ReqCreateUser) -> Result<u32> {
    let user = User {
        phone: ruser.phone,
        password: md5(&ruser.password, "124456"),
    };
    let user_info = UserInfo {
        nickname: ruser.nickname,
        avatar: ruser.avatar,
        gender: ruser.gender,
        birthday: ruser.birthday,
    };

    let uid = create_user_model((user, user_info)).await?;
    Ok(uid)
}

pub async fn login_user_service(rlogin: ReqLogin) -> Result<String> {
    let user = User {
        phone: rlogin.phone,
        password: md5(&rlogin.password, "124456"),
    };
    let res = find_user(user).await?;
    if res.is_none() {
        return Ok("cant find this user.".to_string());
    }
    let c = Claims {
        uid: res.unwrap().uid as u32,
        exp: (chrono::Local::now() + chrono::Duration::minutes(20)).timestamp() as usize,
        iat: chrono::Local::now().timestamp() as usize,
    };
    let token = crate::utils::jwt::create_jwt_token(&c)?;
    Ok(token)
}
