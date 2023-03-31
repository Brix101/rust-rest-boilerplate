use axum::http::Request;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::server::error::Error;

/// Validate User Request.
pub struct ValidationExtractor<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: http_body::Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidationExtractor(value))
    }
}
