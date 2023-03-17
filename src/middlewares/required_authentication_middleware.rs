use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::Extension;
use http::header::AUTHORIZATION;
use tracing::error;

use crate::utils::errors::CustomError;
use crate::utils::jwt_utils::DynJwtUtils;

/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub i64);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = CustomError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(token_service): Extension<DynJwtUtils> =
            Extension::from_request_parts(parts, state)
                .await
                .map_err(|err| CustomError::InternalServerErrorWithContext(err.to_string()))?;

        if let Some(authorization_header) = parts.headers.get(AUTHORIZATION) {
            let header_value = authorization_header
                .to_str()
                .map_err(|_| CustomError::Unauthorized)?;

            if !header_value.contains("Bearer") {
                error!("request does not contain valid 'Bearer' prefix for authorization");
                return Err(CustomError::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                error!("request does not contain a valid token");
                return Err(CustomError::Unauthorized);
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();
            let user_id = token_service
                .get_user_id_from_token(String::from(token_value))
                .map_err(|err| {
                    error!("could not validate user ID from token: {:?}", err);
                    CustomError::Unauthorized
                })?;

            Ok(RequiredAuthentication(user_id))
        } else {
            Err(CustomError::Unauthorized)
        }
    }
}
