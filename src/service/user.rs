use anyhow::{Ok, Result};
use crate::{handler::user::ReqCreateUser, model::user::create_user_model, utils::crypto::md5};
use crate::model::user::{User, UserInfo};

pub async fn create_user_service(ruser: ReqCreateUser) -> Result<u32> {
   let user=User{
       phone:ruser.phone,
       password:md5(&ruser.password, "124456")
   };
   let user_info=UserInfo{
       nickname:ruser.nickname,
       avatar:ruser.avatar,
       gender:ruser.gender,
        birthday:ruser.birthday,
    };
    let uid=create_user_model((user,user_info)).await?;
    Ok(uid)


}