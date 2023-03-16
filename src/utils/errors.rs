use std::borrow::Cow;
use std::{collections::HashMap, fmt::Debug};

use axum::extract::rejection::JsonRejection;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;
use tracing::log::error;
use validator::{ValidationErrors, ValidationErrorsKind};

use crate::dto::ApiError;

pub type CustomResult<T> = Result<T, CustomError>;

pub type CustomErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("username or password is incorrect")]
    InvalidLoginAttmpt,
    #[error("user does not have privilege to access this resource")]
    Forbidden,
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    ApplicationStartup(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("{0}")]
    InternalServerErrorWithContext(String),
    #[error("{0}")]
    ObjectConflict(String),
    #[error("unprocessable request has occurred")]
    UnprocessableEntity { errors: CustomErrorMap },
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl CustomError {
    /// Maps `validator`'s `ValidationrErrors` to a simple map of property name/error messages structure.
    pub fn unprocessable_entity(errors: ValidationErrors) -> Response {
        let mut validation_errors = CustomErrorMap::new();

        // roll through the struct errors at the top level
        for (field_property, error_kind) in errors.into_errors() {
            if let ValidationErrorsKind::Field(field_meta) = error_kind.clone() {
                for error in field_meta.into_iter() {
                    validation_errors
                        .entry(Cow::from(field_property))
                        .or_insert_with(Vec::new)
                        .push(error.message.unwrap_or_else(|| {
                            // required validators contain None for their message, assume a default response
                            let params: Vec<Cow<'static, str>> = error
                                .params
                                .iter()
                                .filter(|(key, _value)| key.to_owned() != "value")
                                .map(|(key, value)| {
                                    Cow::from(format!("{} value is {}", key, value.to_string()))
                                })
                                .collect();

                            if params.len() >= 1 {
                                Cow::from(params.join(", "))
                            } else {
                                Cow::from(format!("{} is required", field_property))
                            }
                        }))
                }
            }
            // structs may contain validators on themselves, roll through first-depth validators
            if let ValidationErrorsKind::Struct(meta) = error_kind.clone() {
                // on structs with validation errors, roll through each of the structs properties to build a list of errors
                for (struct_property, struct_error_kind) in meta.into_errors() {
                    if let ValidationErrorsKind::Field(field_meta) = struct_error_kind {
                        for error in field_meta.into_iter() {
                            validation_errors
                                .entry(Cow::from(struct_property))
                                .or_insert_with(Vec::new)
                                .push(error.message.unwrap_or_else(|| {
                                    // required validators contain None for their message, assume a default response
                                    Cow::from(format!("{} is required", struct_property))
                                }));
                        }
                    }
                }
            }
        }

        let body = Json(json!({
            "error": validation_errors,
        }));

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        if let Self::ValidationError(e) = self {
            return Self::unprocessable_entity(e);
        }

        let (status, error_message) = match self {
            Self::InternalServerErrorWithContext(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::ObjectConflict(err) => (StatusCode::CONFLICT, err),
            Self::InvalidLoginAttmpt => (
                StatusCode::BAD_REQUEST,
                Self::InvalidLoginAttmpt.to_string(),
            ),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            Self::AxumJsonRejection(err) => (StatusCode::BAD_REQUEST, err.body_text()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("unexpected error occurred"),
            ),
        };

        // I'm not a fan of the error specification, so for the sake of consistency,
        // serialize singular errors as a map of vectors similar to the 422 validation responses
        let body = Json(ApiError::new(error_message));

        (status, body).into_response()
    }
}
