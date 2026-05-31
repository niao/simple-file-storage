use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub scope: String,
    pub iat: usize,
    pub exp: usize,
}

use tracing::{info, warn};

pub fn create_upload_token(state: &AppState, sub: Option<String>) -> Result<String, AppError> {
    let now = OffsetDateTime::now_utc();
    let sub = sub.unwrap_or_else(|| "internal".to_string());

    let iat = now.unix_timestamp() as usize;
    let exp_time = now + crate::config::JWT_LIFESPAN;
    let exp = exp_time.unix_timestamp() as usize;

    let claims = Claims {
        sub: sub.clone(),
        scope: "upload".to_string(),
        iat,
        exp,
    };

    // 🔥 ЛОГ ДО СОЗДАНИЯ
    info!(
        action = "jwt_create",
        sub = %sub,
        scope = "upload",
        iat = iat,
        exp = exp,
        ttl_seconds = exp - iat,
        "Creating upload JWT"
    );

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(&state.jwt_secret),
    )
    .map_err(|err| {
        // 🔥 ЛОГ ОШИБКИ
        warn!(
            action = "jwt_create_failed",
            error = ?err,
            "JWT encoding failed"
        );
        AppError::internal("jwt encode failed")
    })?;

    // 🔥 ЛОГ УСПЕХА
    info!(
        action = "jwt_created",
        sub = %sub,
        scope = "upload",
        "JWT successfully created"
    );

    Ok(token)
}

pub fn verify_token(
    state: &AppState,
    token: &str,
    required_scope: &str,
) -> Result<Claims, AppError> {
    tracing::debug!(
        action = "jwt_verify",
        required_scope = %required_scope,
        "Verifying JWT token"
    );

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&state.jwt_secret),
        &validation,
    )
    .map_err(|e| {
        tracing::warn!(
            action = "jwt_verify_failed",
            error = ?e,
            "Token decoding failed"
        );
        AppError::unauthorized("invalid token")
    })?;

    if data.claims.scope != required_scope {
        tracing::warn!(
            action = "jwt_wrong_scope",
            expected = %required_scope,
            actual = %data.claims.scope,
            "Token scope mismatch"
        );
        return Err(AppError::forbidden("wrong scope"));
    }

    tracing::info!(
        action = "jwt_verified",
        sub = %data.claims.sub,
        scope = %data.claims.scope,
        "JWT successfully verified"
    );

    Ok(data.claims)
}

// pub fn verify_token(
//     state: &AppState,
//     token: &str,
//     required_scope: &str,
// ) -> Result<Claims, AppError> {
//     let mut validation = Validation::new(Algorithm::HS256);
//     validation.validate_exp = true;
//
//     let data = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(&state.jwt_secret),
//         &validation,
//     )
//     .map_err(|_| AppError::unauthorized("invalid token"))?;
//
//     if data.claims.scope != required_scope {
//         return Err(AppError::forbidden("wrong scope"));
//     }
//
//     Ok(data.claims)
// }
