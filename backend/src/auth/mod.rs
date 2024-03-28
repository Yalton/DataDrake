use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use std::env;

pub struct Auth;

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<String>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        let auth_header = headers.get("Authorization").and_then(|value| value.to_str().ok());

        let expected_token = env::var("DATA_DRAKE_AUTH_TOKEN").unwrap_or_else(|_| "".to_string());

        match auth_header {
            Some(token) if token == expected_token => Ok(Auth),
            _ => Err((
                StatusCode::UNAUTHORIZED,
                Json("Unauthorized".to_string()),
            )),
        }
    }
}