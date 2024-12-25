use super::error::Error;
use actix_web::{
    get,
    http::{
        header::{ContentLength, ContentType},
        StatusCode,
    },
    post, put,
    web::{self, Data, Form, Header, Json, Path, Query},
    HttpRequest, HttpResponse,
};
use futures::StreamExt;
use hdc_shared::models::signal_data::IngestionResponse;
use log::*;
use serde::{Deserialize, Serialize};
use std::io;
use surrealdb::error::Api;

use crate::sdb::SDBRepository;

use hdc_shared::models::weather_model::WeatherResponse;
#[post("v1/weather/ingest")]
pub async fn ingest_weather(
    sdb_repo: Data<SDBRepository>,
    content_length: Header<ContentLength>,
    data: Form<WeatherResponse>,
    req: HttpRequest,
) -> Result<Json<String>, actix_web::Error> {
    let instance: &str = req.path();
    let meta = sdb_repo
        .get_weather_meta(req.peer_addr().unwrap().ip().to_string(), instance)
        .await;
    let data_points = data.into_inner().to_ingestion_packet(meta?.get_signals());
    match sdb_repo.ingest_data(data_points).await {
        IngestionResponse::Success => Ok(Json("Success".to_string())),
        IngestionResponse::MultiStatus(response) => Ok(Json(response.to_string())),
    }
}

