use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::sea_query::Expr;
use crate::entities::wb_follower;
use crate::model::get_db;


pub async fn find_follower_ids_model(followee: u32) -> Result<Vec<i32>, DbErr> {
   let db = get_db();
   let uids=wb_follower::Entity::find()
       .filter(wb_follower::Column::FolloweeId.eq(followee))
       .all(db)
       .await?;
    if uids.is_empty() {
        return Ok(vec![]);
    }
    let arr: Vec<i32> = uids.iter().map(| v | v.follower_id ).collect();
    Ok(arr)


}

pub async fn find_follower_model(follower: u32, followee: u32) -> Result<Option<wb_follower::Model>, DbErr> {
    let db = get_db();
    let res= wb_follower::Entity::find()
        .filter(
            sea_orm::Condition::all()
                .add(wb_follower::Column::FollowerId.eq(follower))
                .add(wb_follower::Column::FolloweeId.eq(followee))
        )
        .one(db)
        .await?;
    Ok(res)
}

pub async fn create_follow_model(follower: i32, followee: i32) -> Result<(), DbErr> {
    let db = get_db();
    let now = chrono::Local::now().timestamp();
    let user = wb_follower::ActiveModel{
        id: sea_orm::ActiveValue::NotSet,
        follower_id: sea_orm::ActiveValue::Set(follower),
        followee_id: sea_orm::ActiveValue::Set(followee),
        status: sea_orm::ActiveValue::Set(0),
        created_time: sea_orm::ActiveValue::Set(now.try_into().unwrap()) ,
        updated_time: sea_orm::ActiveValue::Set(now.try_into().unwrap()),
    };
    let _ = user.insert(db).await?;
    Ok(())
}

pub async fn delete_follow_model(follower: u32, followee: u32) -> Result<(), DbErr> {
    let db = get_db();
    let res = wb_follower::Entity::delete_many()
        .filter(
            sea_orm::Condition::all()
                .add(wb_follower::Column::FollowerId.eq(follower))
                .add(wb_follower::Column::FolloweeId.eq(followee))
        )
        .exec(db)
        .await?;
    if res.rows_affected == 0 {
        return Err(DbErr::Custom("删除失败，未找到对应记录".to_string()));
    }
      Ok(())
}
pub async fn update_follow_model(follower: u32, followee: u32, status: i16) -> Result<(), DbErr> {
    let db = get_db();
    let now = chrono::Local::now().timestamp() as i32;
    wb_follower::Entity::update_many()
    .col_expr(wb_follower::Column::Status, Expr::value(status))
    .col_expr(wb_follower::Column::UpdatedTime, Expr::value(now))
        .filter(
            sea_orm::Condition::all()
                .add(wb_follower::Column::FollowerId.eq(follower))
                .add(wb_follower::Column::FolloweeId.eq(followee))
        )
        .exec(db)
        .await?;
    Ok(())
}