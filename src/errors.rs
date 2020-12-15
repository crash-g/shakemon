use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ExternalServiceError {
    external_service: ExternalService,
    error: FailedRequest,
}

impl ExternalServiceError {
    pub fn from_pokeapi(error: FailedRequest) -> Self {
        Self {
            external_service: ExternalService::POKEAPI,
            error,
        }
    }

    pub fn from_shakespeare_api(error: FailedRequest) -> Self {
        Self {
            external_service: ExternalService::SHAKESPEARE,
            error,
        }
    }
}

#[derive(Debug)]
enum ExternalService {
    POKEAPI,
    SHAKESPEARE,
}

#[derive(Debug)]
pub enum FailedRequest {
    NotFound {
        query: String,
    },
    TooManyRequests,
    ConnectionError {
        cause: String,
    },
    InvalidPayload {
        query: String,
        cause: String,
    },
    UnexpectedStatusCode {
        query: String,
        status_code: StatusCode,
    },
}

impl FailedRequest {
    pub fn not_found(query: String) -> Self {
        FailedRequest::NotFound { query }
    }

    pub fn too_many_requests() -> Self {
        FailedRequest::TooManyRequests
    }

    pub fn connection_error<C>(cause: C) -> Self
    where
        C: Debug,
    {
        FailedRequest::ConnectionError {
            cause: format!("{:?}", cause),
        }
    }

    pub fn invalid_payload<C>(query: String, cause: C) -> Self
    where
        C: Debug,
    {
        FailedRequest::InvalidPayload {
            query,
            cause: format!("{:?}", cause),
        }
    }

    pub fn unexpected_status_code(query: String, status_code: StatusCode) -> Self {
        FailedRequest::UnexpectedStatusCode { query, status_code }
    }
}

impl std::fmt::Display for ExternalServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.external_service, self.error)
    }
}

impl ResponseError for ExternalServiceError {
    fn error_response(&self) -> HttpResponse {
        match &self.error {
            FailedRequest::NotFound { query } => {
                log::debug!(
                    "External service {:?} returned NOT_FOUND. Original query: {}",
                    self.external_service,
                    query
                );
                HttpResponse::NotFound().finish()
            }
            FailedRequest::TooManyRequests => {
                log::debug!(
                    "External service {:?} returned TOO_MANY_REQUESTS",
                    self.external_service
                );
                HttpResponse::TooManyRequests().finish()
            }
            FailedRequest::ConnectionError { cause } => {
                log::error!(
                    "Connection error with external service {:?} failed. Reason: {}",
                    self.external_service,
                    cause
                );
                HttpResponse::InternalServerError().finish()
            }
            FailedRequest::InvalidPayload { query, cause } => {
                log::error!(
                    "Cannot deserialize JSON payload from external service {:?}. \
                     Error: {}. Original query: {}",
                    self.external_service,
                    cause,
                    query
                );
                HttpResponse::InternalServerError().finish()
            }
            FailedRequest::UnexpectedStatusCode { query, status_code } => {
                log::error!(
                    "An unexpected status code has been received from external service {:?}. \
                     Status code: {}. Original query: {}",
                    self.external_service,
                    status_code,
                    query
                );
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
