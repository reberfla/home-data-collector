#![allow(unused)]

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use crate::prelude::*;

mod error;
mod prelude;
mod utils;
mod api;
mod model;
mod repository;


use api::signal::{
    register_signal, get_signal_all, get_signal
};
use api::ingest::ingest;
use api::query_timeseries::query_timeseries;
use repository::sdb::{SDBRepository, self};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUTS_BACKTRACE", "1");
    env_logger::init();

    let mut client: Surreal<Client> = Surreal::new::<Ws>("192.168.0.240:80").await.expect("Can't connect to SurrealBD instance!");
    client.signin(Root {
        username: "root",
        password: "root"
    }).await.unwrap();
    client.use_ns("test").use_db("test").await.unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();
        let sdb_repo: SDBRepository = SDBRepository::init(client.clone());
        let sdb_data = Data::new(
            sdb_repo
        );
        App::new()
        .wrap(logger)
        .app_data(sdb_data)
        .service(register_signal)
        .service(get_signal_all) 
        .service(get_signal)
        .service(ingest)
        .service(query_timeseries)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}