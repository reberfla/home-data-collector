use actix_web::{
    Error,
    error,
    post,
    get,
    web::{Data, Json, JsonBody, Payload, Header, BytesMut},
    web,
    HttpResponse,
    http::{header::{ContentType, ContentLength}, StatusCode}};
use futures::StreamExt;
use serde::{Serialize, Deserialize};

use crate::app::signal_data::model::{IngestionPacket, IngestionResponse, MultiStatusData,
                                    QueryResponse, QueryResult, QueryTimeseriesData};
use crate::app::signal_data::error::QueryError;
use crate::sdb::SDBRepository;


#[post("v1/ingest")]
pub async fn ingest(sdb_repo: Data<SDBRepository>, mut payload: Payload,
                    content_length: Header<ContentLength>) -> Result<Json<String>, Error>{
    let mut body = BytesMut::new();
    let body_length = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > body_length {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let data_points = serde_json::from_slice::<IngestionPacket>(&body)?;
    match sdb_repo.ingest_data(data_points).await {
        IngestionResponse::Success => Ok(Json("Success".to_string())),
        IngestionResponse::MultiStatus(response) => Err(response.into())
    }
}

#[get("v1/query/timeseriesdata")]
pub async fn query_timeseries(sdb_repo: Data<SDBRepository>, mut payload: Payload,
                              content_length: Header<ContentLength>) -> Result<Json<QueryResult>, Error>{
    let mut body = BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > body_length{
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let query = serde_json::from_slice::<QueryTimeseriesData>(&body)?;
    match sdb_repo.query_timeseries(query).await {
        QueryResponse::Success(response) => Ok(Json(response)),
        QueryResponse::Failed => return Err(QueryError::Failed.into())
    }
}
