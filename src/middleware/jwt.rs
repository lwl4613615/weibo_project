use axum::{extract::FromRequestParts, http::request::Parts, response::Response};
use axum_extra::typed_header::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;


use crate::utils::{jwt::{Claims, decode_jwt_token}, result::response_json};

#[derive(Clone)]
pub struct AuthClaims(pub Claims); // Claims 是你定义的 JWT 结构


impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. 取出 Authorization: Bearer xxx
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| response_json(5, ()))?;

        // 2. 验证/解码 JWT
        let token_data = decode_jwt_token(bearer.token()).map_err(|_| response_json(6, ()))?;

        Ok(AuthClaims(token_data))
    }
}