use anyhow::{anyhow, Result};

use crate::model::{
    follower::{create_follow_model, find_follower_model, update_follow_model},
    user::find_by_uid_model,
};

pub async fn follower_service(follower_id: u32, followee_id: u32) -> Result<()> {
  
    find_by_uid_model(followee_id)
        .await?
        .ok_or_else(|| anyhow!("用户不存在"))?;

    match find_follower_model(follower_id, followee_id).await? {
        Some(record) => {
            if record.status != 0 {
                update_follow_model(follower_id, followee_id, 0).await?;
            }
        }
        None => {
            create_follow_model(follower_id as i32, followee_id as i32).await?;
        }
    }

    Ok(())
}

pub async fn unfollower_service(follower_id: u32, followee_id: u32) -> Result<()> {
   
    match find_follower_model(follower_id, followee_id).await? {
        Some(record) => {
            if record.status == 0 {
                update_follow_model(follower_id, followee_id, 1).await?;
                
            }
        }
        None => {
            
            return Err(anyhow!("未关注该用户，无法取消关注"));
        }
    }

    Ok(())
}