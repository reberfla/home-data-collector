use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use crate::config::ServerConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{opt::auth::Database, Surreal};

#[derive(Clone)]
pub struct SDBRepository {
    pub db: Surreal<Client>,
}

impl SDBRepository {
    pub async fn init(config: &ServerConfig) -> Self {
        let mut client: Surreal<Client> =
            Surreal::new::<Ws>(format!("{}:{}", config.db_address, config.db_port))
                .await
                .expect("Can't connect to SurrealBD instance!");
        client
            .signin(Database {
                username: &config.db_username,
                password: &config.db_password,
                namespace: &config.db_namespace,
                database: &config.db_database,
            })
            .await
            .unwrap();
        SDBRepository { db: client }
    }
}
