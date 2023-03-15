// use async_trait::async_trait;
// use axum::extract::FromRequest;
// use axum::http::Request;
// use axum::{BoxError, Json};
// use serde::de::DeserializeOwned;
// use validator::Validate;

// use crate::core::errors::CustomError;
// /// use this to encapsulate fields that require validation
// #[derive(Debug, Clone, Copy, Default)]
// pub struct ValidationExtractor<T>(pub T);

// #[async_trait]
// impl<S, T, B> FromRequest<S, B> for ValidationExtractor<T>
// where
//     T: FromRequest<S, B>,
//     B: Send + 'static,
//     S: Send + Sync,
// {
//     type Rejection = CustomError;

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(req, state).await?;
//         value.validate()?;
//         Ok(ValidationExtractor(value))
//     }
// }
