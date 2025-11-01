use tokio::sync::OnceCell;

#[derive(Debug, serde::Serialize, serde::Deserialize,Clone)]
pub struct Claims {
    pub uid: u32,
    pub exp: usize,
    pub iat: usize,
}

pub static JWTSECRET_INFO: OnceCell<JwtSecret> = OnceCell::const_new();
pub struct JwtSecret {
    pub private_key: String,
    pub public_key: String,
}

pub async fn init_jwt_secret() {
    JWTSECRET_INFO
        .get_or_init(|| async {
            let mut path = std::env::current_dir().unwrap();
            path.push("private.pem");
            let private_key = tokio::fs::read_to_string(path).await.unwrap();
            let mut path = std::env::current_dir().unwrap();
            path.push("public.pem");
            let public_key = tokio::fs::read_to_string(path).await.unwrap();
            JwtSecret {
                private_key,
                public_key,
            }
            
        })
        .await;
}

pub fn create_jwt_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{EncodingKey, Header, encode};
    let encoding_key = EncodingKey::from_rsa_pem(JWTSECRET_INFO.get().unwrap().private_key.as_bytes())?;
    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::RS512),
        claims,
        &encoding_key,
    )?;
    Ok(token)
}

pub fn decode_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{DecodingKey, Validation, decode};
    let decoding_key = DecodingKey::from_rsa_pem(JWTSECRET_INFO.get().unwrap().public_key.as_bytes())?;
    let token_data = decode::<Claims>(
        token,
        &decoding_key,
        &Validation::new(jsonwebtoken::Algorithm::RS512),
    )?;
    
    Ok(token_data.claims)
}
