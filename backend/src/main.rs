#![allow(unused)]
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::{debug, info, warn};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

mod app;
mod config;
mod error;
mod sdb;

use app::interface::route::*;
use app::signal_data::route::*;
use app::weather_connector::route::*;
use config::ServerConfig;
use sdb::SDBRepository;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: ServerConfig = ServerConfig::load("./Config.yml");

    std::env::set_var("RUST_LOG", &config.loglevel);
    env_logger::init();
    debug!("{}", config);

    let sdb_repo: SDBRepository = SDBRepository::init(&config).await;

    HttpServer::new(move || {
        let logger = Logger::default();
        let sdb_data = Data::new(sdb_repo.clone());
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(sdb_data)
            .service(ingest_ts_data)
            .service(query_timeseries)
            .service(register_interface)
            .service(update_interface)
            .service(get_tasks)
            .service(query_interface)
            .service(ingest_weather)
    })
    .bind(
        (SocketAddr::new(
            IpAddr::from_str(&config.listen_address).unwrap(),
            config.listen_port,
        )),
    )?
    .run()
    .await
}
