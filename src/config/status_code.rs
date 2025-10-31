use std::collections::HashMap;

use tokio::sync::OnceCell;

pub static STATUS_CODE_MSG: OnceCell<HashMap<usize, &str>> = OnceCell::const_new();

pub async fn init_status_code() {
    STATUS_CODE_MSG
        .get_or_init(|| async {
            let mut map = HashMap::new();
            map.insert(0, "Success");
            map.insert(1, "Failed");
            map.insert(2, "手机验证失败,格式不正确");
            map.insert(
                3,
                "你的密码长度不符合要求,请重新设置,密码长度应在6-20位之间",
            );
             map.insert(
                4,
                "用户信息错误，请使用正确的用户名和密码登录",
            );
             map.insert(
                5,
                "缺少TOKEN信息，请在请求头中添加Authorization字段",
            );
            map.insert(
                6,
                "TOKEN验证失败，请重新登录",
            );
            map
        })
        .await;
}
