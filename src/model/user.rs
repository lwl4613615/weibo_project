use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, DbErr,  TransactionTrait};

use crate::{entities::{wb_user, wb_user_info},  model::get_db, utils::time::local_timestamp};

pub struct User{
    pub phone:String,
    pub password:String,
}
pub struct UserInfo{      
    pub nickname:Option<String>,
    pub avatar:Option<String>,
    pub gender:Option<i16>,
    pub birthday:Option<i32>
}

pub async fn create_user_model((u,uf):(User,UserInfo)) -> Result<u32,DbErr> {


    let db = get_db();
    let txn = db.begin().await?;
    let now = local_timestamp() as i32;

    let user = wb_user::ActiveModel {
        uid: NotSet,
        phone: Set(u.phone),
        password: Set(u.password),
        created_time: Set(Some(now)),
        updated_time: Set(now),
    };

    let res = match user.insert(&txn).await {
        Ok(model) => model,
        Err(err) => {
            txn.rollback().await?;
            return Err(sea_orm::DbErr::Custom(format!("创建用户失败: {err}")));
        }
    };
    let uid = res.uid;

    let user_info = wb_user_info::ActiveModel {
        uid: Set(uid),
        nickname: Set(uf.nickname),
        avatar: Set(uf.avatar),
        gender: Set(uf.gender),
        birthday: Set(uf.birthday),
        updated_time: Set(now),
    };

    if let Err(err) = user_info.insert(&txn).await {
        txn.rollback().await?;
        return Err(sea_orm::DbErr::Custom(format!("创建用户详情失败: {err}")));
    }

    txn.commit().await?;

    Ok(uid as u32)

}