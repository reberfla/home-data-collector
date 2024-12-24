use super::error::Error;
use super::model::InterfaceQuery;
use actix_web::{
    get,
    http::{
        header::{ContentLength, ContentType},
        StatusCode,
    },
    post, put,
    web::{self, Data, Header, Json, Path, Query},
    HttpRequest, HttpResponse,
};
use futures::StreamExt;
use log::*;
use serde::{Deserialize, Serialize};
use std::io;
use surrealdb::error::Api;

use crate::sdb::SDBRepository;

use hdc_shared::models::{
    interface::Interface,
    tasklist::{CollectorTask, Tasklist},
};
#[post("v1/interface/register")]
pub async fn register_interface(
    sdb_repo: Data<SDBRepository>,
    mut payload: web::Payload,
    content_length: Header<ContentLength>,
    req: HttpRequest,
) -> Result<Json<String>, Error> {
    let instance: &str = req.path();
    let mut body = web::BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|e| Error::Payload {
            error: e,
            instance: instance.to_owned(),
        })?;

        if (body.len() + chunk.len()) > body_length {
            return Err(Error::BodyOverflow {
                instance: instance.to_owned(),
            });
        }
        body.extend_from_slice(&chunk);
    }
    let mut interface =
        serde_json::from_slice::<Interface>(&body).map_err(|error| Error::Json {
            error,
            instance: instance.to_owned(),
        })?;
    interface.add_uuid();

    let response = sdb_repo.register_interface(interface, instance).await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(e) => {
            info!("{}", e);
            Err(e)
        }
    }
}

#[get("v1/tasks")]
pub async fn get_tasks(
    sdb_repo: Data<SDBRepository>,
    req: HttpRequest,
) -> Result<Json<Tasklist>, Error> {
    let instance = req.path();
    let response: Result<Vec<CollectorTask>, Error> = sdb_repo.get_tasks(instance).await;
    match response {
        Ok(response) => Ok(Json(Tasklist { tasks: response })),
        Err(e) => {
            info!("{}", e);
            Err(e)
        }
    }
}

#[get("v1/interface")]
pub async fn query_interface(
    sdb_repo: Data<SDBRepository>,
    query: Query<InterfaceQuery>,
    req: HttpRequest,
) -> Result<Json<Vec<Interface>>, Error> {
    let instance: &str = req.path();
    let response: Result<Vec<Interface>, Error> = sdb_repo
        .query_interfaces(query.clone().into_inner(), instance)
        .await;
    info!("{:?}", query.into_inner());
    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            info!("{}", e);
            Err(e)
        }
    }
}

#[put("v1/interface/update/{uuid}")]
pub async fn update_interface(
    sdb_repo: Data<SDBRepository>,
    mut payload: web::Payload,
    content_length: Header<ContentLength>,
    interface_uuid: Path<String>,
    req: HttpRequest,
) -> Result<Json<String>, Error> {
    let instance = req.path();
    let mut body = web::BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|e| Error::Payload {
            error: e,
            instance: instance.to_owned(),
        })?;

        if (body.len() + chunk.len()) > body_length {
            return Err(Error::BodyOverflow {
                instance: instance.to_owned(),
            });
        }
        body.extend_from_slice(&chunk);
    }
    let mut interface =
        serde_json::from_slice::<Interface>(&body).map_err(|e| Error::Json {
            error: e,
            instance: instance.to_owned(),
        })?;

    let response = sdb_repo
        .update_interface(interface, interface_uuid.into_inner(), instance)
        .await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(e) => {
            info!("{}", e);
            Err(e)
        }
    }
}
