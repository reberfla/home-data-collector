use super::super::problem_details::ProblemDetails;
use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use derive_more::From;
use hdc_shared::models::signal_data::MultiStatusData;
use serde::Serialize;
use serde_with::SerializeDisplay;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    BodyOverflow {
        instance: String,
    },
    UnknownLength {
        instance: String,
    },
    CannotParse {
        instance: String,
    },
    QueryFailed {
        instance: String,
    },
    //MalformedQuery{instance: String} --> do when implementing logic to check invalid args,
    SomethingWrong {
        instance: String,
    },
    #[from]
    Db {
        error: surrealdb::Error,
        instance: String,
    },

    #[from]
    Json {
        error: serde_json::Error,
        instance: String,
    },
    #[from]
    PartialIngestion {
        instance: String,
        data: MultiStatusData,

    },
    #[from]
    Payload {
        error: actix_web::error::PayloadError,
        instance: String,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&self.to_problemdetails()).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.map_status_code()).unwrap()
    }
}

impl Error {
    fn to_problemdetails(&self) -> ProblemDetails {
        let status_code: u16 = self.map_status_code();
        match self {
            Error::BodyOverflow { instance } => ProblemDetails::new(
                None,
                status_code,
                "Unable to Parse body".to_owned(),
                "Body longer than content-length header".to_string(),
                instance.to_string(),
            ),
            Error::UnknownLength { instance } => ProblemDetails::new(
                None,
                status_code,
                "Unable to Parse body".to_owned(),
                "Length of body is not known".to_owned(),
                instance.to_string(),
            ),
            Error::CannotParse { instance } => ProblemDetails::new(
                None,
                status_code,
                "Unable to Parse body".to_owned(),
                "An error occured during the parsing of the body.".to_string(),
                instance.to_string(),
            ),
            Error::QueryFailed { instance } => ProblemDetails::new(
                None,
                status_code,
                "Query Failed".to_owned(),
                "An error occured during the execution of the query.".to_string(),
                instance.to_string(),
            ),
            Error::SomethingWrong { instance } => ProblemDetails::new(
                None,
                status_code,
                "Something went wrong".to_owned(),
                "An unknown error occured".to_string(),
                instance.to_owned(),
            ),
            Error::Json { error, instance } => ProblemDetails::new(
                None,
                status_code,
                "Unable to Parse body".to_owned(),
                error.to_string(),
                instance.to_string(),
            ),
            Error::Payload { error, instance } => ProblemDetails::new(
                None,
                status_code,
                "Payload Error".to_owned(),
                error.to_string(),
                instance.to_string(),
            ),
            Error::Db { error, instance } => ProblemDetails::new(
                None,
                status_code,
                "Database Error".to_owned(),
                error.to_string(),
                instance.to_string(),
            ),
            Error::PartialIngestion { instance, data } => ProblemDetails::new(
                None,
                status_code,
                "Partial Ingestion".to_owned(),
                serde_json::to_string(&data).unwrap(),
                instance.to_string(),
            ),
            _ => ProblemDetails::new(
                None,
                status_code,
                "Something went wrong".to_owned(),
                "An unknown error occured".to_string(),
                "unknown".to_string(),
            ),
        }
    }
    fn map_status_code(&self) -> u16 {
        match self {
            Error::SomethingWrong { .. } => 500,
            Error::PartialIngestion { .. } => 207,
            _ => 400,
        }
    }
}
