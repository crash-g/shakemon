use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum CustomError {
    NotFound,
    InternalServerError,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("An internal server error occurred")?;
        Ok(())
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        use CustomError::*;
        match self {
            NotFound => HttpResponse::NotFound().finish(),
            InternalServerError => HttpResponse::InternalServerError().finish(),
        }
    }
}
