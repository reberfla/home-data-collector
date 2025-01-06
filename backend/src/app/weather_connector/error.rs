use super::super::problem_details::ProblemDetails;
use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use derive_more::From;
use serde::Serialize;
use serde_with::SerializeDisplay;
use hdc_shared::models::signal_data::MultiStatusData;

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
    //MalformedQuery{instance: String} --> do when implementing logic to check invalid args,
    WeatherConnectorAlreadyExists {
        instance: String,
    },
    UpdateUuidNotAllowed {
        instance: String,
    },
    UpdateWeatherConnectortypeNotAllowed {
        instance: String,
    },
    UpdateWeatherConnectorNotExists {
        instance: String,
    },
    UpdateFailed {
        instance: String,
    },
    SomethingWrong {
        instance: String,
    },
    #[from]
    PartialIngestion {
        instance: String,
        data: MultiStatusData,

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
        StatusCode::from_u16(self.get_status_code()).unwrap()
    }
}

impl Error {
    fn to_problemdetails(&self) -> ProblemDetails {
        let status_code: u16 = self.get_status_code();
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
            Error::WeatherConnectorAlreadyExists { instance } => ProblemDetails::new(
                None,
                status_code,
                "Database Error".to_owned(),
                "WeatherConnector Already exists".to_owned(),
                instance.to_owned(),
            ),
            Error::UpdateUuidNotAllowed { instance } => ProblemDetails::new(
                None,
                status_code,
                "UUID update not allowed".to_owned(),
                "Updating UUID's of any interface or signal is not allowed".to_owned(),
                instance.to_owned(),
            ),
            Error::UpdateWeatherConnectortypeNotAllowed { instance } => ProblemDetails::new(
                None,
                status_code,
                "update interface type not allowed".to_owned(),
                "You tried to modify the interface_type which is not allowed".to_owned(),
                instance.to_owned(),
            ),
            Error::UpdateWeatherConnectorNotExists { instance } => ProblemDetails::new(
                None,
                status_code,
                "WeatherConnector does not exist.".to_owned(),
                "The interface you tried to update does not exists.".to_owned(),
                instance.to_owned(),
            ),
            Error::UpdateFailed { instance } => ProblemDetails::new(
                None,
                status_code,
                "Updating the interface failed.".to_owned(),
                "An error occured during updating the interface in the database.".to_owned(),
                instance.to_owned(),
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
    fn get_status_code(&self) -> u16 {
        match self {
            Error::UpdateUuidNotAllowed { .. } => 403,
            Error::WeatherConnectorAlreadyExists { .. } => 403,
            Error::UpdateUuidNotAllowed { .. } => 403,
            Error::UpdateWeatherConnectortypeNotAllowed { .. } => 403,
            Error::UpdateWeatherConnectorNotExists { .. } => 403,
            Error::UpdateFailed { .. } => 500,
            Error::SomethingWrong { .. } => 500,
            Error::PartialIngestion { .. } => 207,
            _ => 400,
        }
    }
}
