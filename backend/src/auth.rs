use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // User ID
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[allow(dead_code)]
pub struct AuthUser {
    pub user_id: i32,
    pub username: String,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| format!("Failed to hash password: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

pub fn create_jwt(user_id: i32, username: &str, secret: &str) -> Result<String, String> {
    let now = Utc::now();
    let expire = now + Duration::days(30);

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Failed to generate authentication token: {}", e))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, String> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid or expired authentication token: {}", e))
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "fivebx_development_secret_key_change_in_production!".to_string());

        // 1. Check HTTP-only cookie 'fivebx_session'
        let jar = CookieJar::from_request_parts(parts, _state)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorised: Invalid cookie jar").into_response())?;

        let mut token_str = jar.get("fivebx_session").map(|c| c.value().to_string());

        // 2. Fallback to Authorization: Bearer <token>
        if token_str.is_none() {
            if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Some(stripped) = auth_str.strip_prefix("Bearer ") {
                        token_str = Some(stripped.trim().to_string());
                    }
                }
            }
        }

        let token = match token_str {
            Some(t) => t,
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "error": "Authentication required. Please log in to proceed."
                    })),
                )
                    .into_response());
            }
        };

        match verify_jwt(&token, &secret) {
            Ok(claims) => Ok(AuthUser {
                user_id: claims.sub,
                username: claims.username,
            }),
            Err(_) => Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Your session has expired or is invalid. Please log in again."
                })),
            )
                .into_response()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hash() {
        let h = hash_password("FiveBX2026!").unwrap();
        println!("GENERATED_HASH:{}", h);
        assert!(verify_password("FiveBX2026!", &h));
    }
}
