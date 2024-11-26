use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Display)]
#[display("{:?}{}{}{}{}", error_type, status, title, detail, instance)]
pub struct ProblemDetails {
    #[serde(rename(serialize = "type"), skip_serializing_if = "Option::is_none")]
    error_type: Option<String>,
    status: u16,
    title: String,
    detail: String,
    instance: String,
}

impl ProblemDetails {
    pub fn internal_server_error(url_path: String) -> Self {
        Self {
            error_type: None,
            status: 500,
            title: "Internal Server Error".to_owned(),
            detail: "Server was not able to give a proper error response".to_owned(),
            instance: url_path,
        }
    }
    pub fn status_code(&self) -> u16 {
        self.status
    }
    pub fn new(
        error_type: Option<String>,
        status: u16,
        title: String,
        detail: String,
        instance: String,
    ) -> Self {
        ProblemDetails {
            error_type,
            status,
            title,
            detail,
            instance,
        }
    }
}
