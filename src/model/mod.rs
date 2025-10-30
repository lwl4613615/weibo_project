pub mod user;
use sea_orm::{Database, DbConn};
use tokio::sync::OnceCell;

use crate::config::app_config::APP_CONFIGS;

pub static DB_CONN:OnceCell<DbConn>=OnceCell::const_new();

pub async fn db_conn_init()
{
    DB_CONN.get_or_init(||async{
        let db=&APP_CONFIGS.get().unwrap().db;
        let db_url=format!("postgresql://{}:{}@{}:{}/{}",db.username,db.password,db.host,db.port,db.database_name);
        let db_conn=Database::connect(db_url).await.expect("Database connect error");
        db_conn
    }).await;
}


pub fn get_db()->&'static DbConn
{
   let db= DB_CONN.get().unwrap();
    db
}