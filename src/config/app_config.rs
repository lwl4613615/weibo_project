use serde::Deserialize;
use tokio::sync::OnceCell;

pub static APP_CONFIGS: OnceCell<Appconfig> = OnceCell::const_new();

pub async fn app_init() {
    APP_CONFIGS
        .get_or_init(|| async {
            let mut path = std::env::current_dir().unwrap();
            path.push("app.toml");
            let config_str = tokio::fs::read_to_string(path)
                .await
                .expect("get read configfile");
            let config: Appconfig = toml::from_str(&config_str).unwrap();
            config
        })
        .await;
}

#[derive(Debug, Deserialize)]
pub struct Appconfig {
    pub db: Dbconfig,
    pub cache: Cacheconfig,
    pub jwt: Jwtconfig,
}
#[derive(Debug, Deserialize)]
pub struct Dbconfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}
#[derive(Debug, Deserialize)]
pub struct Cacheconfig {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Deserialize)]
pub struct Jwtconfig {
    pub secret: String,
}
