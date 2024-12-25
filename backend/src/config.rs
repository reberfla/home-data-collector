use core::panic;
use regex::Captures;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env::VarError;
use std::fs;
use std::io;
use std::u16;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct ServerConfigYml {
    logging: Option<Logging>,
    listen_parameters: Option<Endpoint>,
    database: Option<Database>,
}

#[derive(Serialize, Deserialize)]
struct Logging {
    loglevel: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Endpoint {
    address: Option<String>,
    port: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct Database {
    address: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    namespace: Option<String>,
    database: Option<String>,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub loglevel: String,
    pub listen_address: String,
    pub listen_port: u16,
    pub db_address: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_namespace: String,
    pub db_database: String,
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ServerConfig {{ loglevel: {}, listen_address: {}, listen_port: {}, db_address: {}, db_port: {}, db_username: {}, db_namespace: {}, db_database: {} }}",
            self.loglevel, self.listen_address, self.listen_port, self.db_address, self.db_port, self.db_username, self.db_namespace, self.db_database
        )
    }
}

impl ServerConfig {
    pub fn load(config_filepath: &str) -> Self {
        let mut content: String = String::new();

        let config: io::Result<String> = fs::read_to_string(config_filepath);

        if config.is_ok() {
            let reg = Regex::new(r"\$\{(?<value>[A-Za-z_]+)\}").unwrap();
            content = config.unwrap();
            content = reg
                .replace_all(&content, |caps: &Captures| {
                    let var_name = &caps["value"];
                    std::env::var(var_name)
                        .unwrap_or_else(|_| panic!("Environment variable {} not set", var_name))
                })
                .to_string();
        }

        let config_yml: ServerConfigYml =
            serde_yml::from_str(&content).unwrap_or_else(|_| ServerConfigYml {
                logging: None,
                listen_parameters: None,
                database: None,
            });

        let loglevel: String = match config_yml.logging {
            Some(logging) => logging.loglevel.unwrap_or_else(|| {
                println!("Missing field loglevel in table logging, taking info as default");
                "debug".to_owned()
            }),
            None => {
                println!("Missing table logging, taking info as default loglevel");
                "debug".to_owned()
            }
        };

        let (listen_address, listen_port): (String, u16) = match config_yml.listen_parameters {
            Some(listen_parameters) => {
                let address: String = listen_parameters.address.unwrap_or_else(|| {
                    println!("Missing field address in table endpoint, taking 0.0.0.0 as default");
                    "0.0.0.0".to_owned()
                });

                let port: u16 = listen_parameters.port.unwrap_or_else(|| {
                    println!("Missing field port in table endpoint, taking 8080 as default");
                    8080
                });
                (address, port)
            }
            None => {
                println!("Missing table endpoint, taking default values");
                ("127.0.0.1".to_owned(), 8080)
            }
        };
        let (db_address, db_port, db_username, db_password, db_namespace, db_database): (
            String,
            u16,
            String,
            String,
            String,
            String,
        ) = match config_yml.database {
            Some(db_param) => {
                let address: String = db_param.address.unwrap_or_else(|| {
                    println!(
                        "Missing field address in table databse, taking 127.0.0.1 as default."
                    );
                    "127.0.0.1".to_string()
                });

                let port: u16 = db_param.port.unwrap_or_else(|| {
                    println!("Missing field port in table database, taking 80 as default.");
                    80
                });

                let username: String = db_param.username.unwrap_or_else(|| {
                    println!("Missing field username in table database, taking default.");
                    "dev".to_owned()
                });

                let password: String = db_param.password.unwrap_or_else(|| {
                    println!("Missing field password in table database, taking default.");
                    "dev".to_owned()
                });

                let namespace: String = db_param.namespace.unwrap_or_else(|| {
                    println!("Missing field namespace in table database, taking default.");
                    "dev".to_owned()
                });

                let database: String = db_param.database.unwrap_or_else(|| {
                    println!("Missing field database in table database, taking default.");
                    "dev".to_owned()
                });
                (address, port, username, password, namespace, database)
            }
            None => {
                println!("Missing table database, taking default values.");

                (
                    "127.0.0.1".to_owned(),
                    80,
                    "dev".to_owned(),
                    "dev".to_owned(),
                    "dev".to_owned(),
                    "dev".to_owned(),
                )
            }
        };
        ServerConfig {
            loglevel,
            listen_address,
            listen_port,
            db_address,
            db_port,
            db_username,
            db_password,
            db_namespace,
            db_database,
        }
    }
}
