use axum::{async_trait, extract::FromRequest, BoxError, Json, RequestExt};
use http::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::errors::CustomError;

pub struct ValidatedRequest<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedRequest<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + 'static,
    B: http_body::Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = CustomError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|_| CustomError::BadRequest("Invalid JSON body".to_string()))?;

        data.validate()
            .map_err(|_| CustomError::BadRequest("Invalid JSON body".to_string()))?;
        Ok(Self(data))
    }
}
