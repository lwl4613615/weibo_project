use std::collections::HashMap;

use tokio::sync::OnceCell;

pub static STATUS_CODE_MSG:OnceCell<HashMap<usize,&str>>=OnceCell::const_new();

pub async fn init_status_code()
{
    STATUS_CODE_MSG.get_or_init(|| async {
        let mut map = HashMap::new();
        map.insert(0,"Success");
        map.insert(1,"Failed");      
        map
    }).await;
}