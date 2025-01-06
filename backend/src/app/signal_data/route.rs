use super::error::Error;
use actix_web::{
    error, get,
    http::{
        header::{ContentLength, ContentType},
        StatusCode,
    },
    post, web,
    web::{BytesMut, Data, Header, Json, JsonBody, Payload},
    HttpRequest, HttpResponse,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::sdb::SDBRepository;
use hdc_shared::models::{
    ingestion_container::IngestionPacket,
    signal_data::{IngestionResponse, QueryResponse, QueryResult, QueryTimeseriesData},
};

#[post("v1/ts/ingest")]
pub async fn ingest_ts_data(
    sdb_repo: Data<SDBRepository>,
    mut payload: Payload,
    content_length: Header<ContentLength>,
    req: HttpRequest,
) -> Result<Json<String>, Error> {
    let instance: &str = req.path();
    let mut body = BytesMut::new();
    let body_length = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|error| Error::Payload {
            error,
            instance: instance.to_owned(),
        })?;

        if (body.len() + chunk.len()) > body_length {
            return Err(Error::BodyOverflow {
                instance: instance.to_owned(),
            });
        }
        body.extend_from_slice(&chunk);
    }
    let data_points =
        serde_json::from_slice::<IngestionPacket>(&body).map_err(|error| Error::Json {
            error,
            instance: instance.to_owned(),
        })?;
    match sdb_repo.ingest_data(data_points).await {
        IngestionResponse::Success => Ok(Json("Success".to_string())),
        IngestionResponse::MultiStatus(response) => Err(Error::PartialIngestion { instance: instance.to_owned(), data: response })
    }
}

#[post("v1/query/timeseriesdata")]
pub async fn query_timeseries(
    sdb_repo: Data<SDBRepository>,
    mut payload: Payload,
    content_length: Header<ContentLength>,
    req: HttpRequest,
) -> Result<Json<QueryResult>, Error> {
    let instance: &str = req.path();
    let mut body = BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|error| Error::Payload {
            error,
            instance: instance.to_owned(),
        })?;

        if (body.len() + chunk.len()) > body_length {
            return Err(Error::BodyOverflow {
                instance: instance.to_owned(),
            });
        }
        body.extend_from_slice(&chunk);
    }
    let query =
        serde_json::from_slice::<QueryTimeseriesData>(&body).map_err(|error| Error::Json {
            error,
            instance: instance.to_owned(),
        })?;
    let result: QueryResult = sdb_repo.query_timeseries(query, instance).await?;
    Ok(Json(result))
}
