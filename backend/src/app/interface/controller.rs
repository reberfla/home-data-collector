use super::error::Error;
use serde::{Deserialize, Serialize};
use surrealdb::sql::statements::OptionStatement;
use surrealdb::{error::Api, sql::Value};

use crate::sdb::SDBRepository;
use hdc_shared::models::signal_meta::SignalMeta;
use hdc_shared::models::{interface::*, tasklist::*};
use log::info;

use super::model::InterfaceQuery;

impl SDBRepository {
    pub async fn register_interface(
        &self,
        interface: Interface,
        instance: &str,
    ) -> Result<(), Error> {
        let mut existing = self
            .db
            .query(format!(
                "SELECT * FROM interface WHERE url = '{}'",
                interface.get_url()
            ))
            .await
            .map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;
        let result: Vec<Interface> = existing.take(0).map_err(|e| Error::Db {
            error: e,
            instance: instance.to_owned(),
        })?;
        match result.len() {
            0 => (),
            _ => {
                return Err(Error::InterfaceAlreadyExists {
                    instance: instance.to_owned(),
                })
            }
        }
        let created: Option<Interface> = self
            .db
            .create(("interface", interface.get_uuid().clone().unwrap()))
            .content(interface)
            .await
            .map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;
        match created {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    pub async fn get_tasks(&self, instance: &str) -> Result<Vec<CollectorTask>, Error> {
        let response: Vec<Interface> =
            self.db.select("interface").await.map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;
        let tasklist = response
            .into_iter()
            .map(|entry| entry.to_task())
            .filter(|entry| entry.is_some())
            .map(|entry| entry.unwrap())
            .collect();
        Ok(tasklist)
    }

    pub async fn query_interfaces(
        &self,
        interface_query: InterfaceQuery,
        instance: &str,
    ) -> Result<Vec<Interface>, Error> {
        let sql: String = interface_query.build_sql_query();
        let mut response = self.db.query(sql).await.map_err(|e| Error::Db {
            error: e,
            instance: instance.to_owned(),
        })?;

        let result: Vec<Interface> = response.take(0).map_err(|e| Error::Db {
            error: e,
            instance: instance.to_owned(),
        })?;
        Ok(result)
    }

    pub async fn update_interface(
        &self,
        interface: Interface,
        uuid: String,
        instance: &str,
    ) -> Result<(), Error> {
        let interface_uuid = interface.get_uuid().unwrap();
        let existing: Option<Interface> =
            self.db
                .select(("interface", uuid))
                .await
                .map_err(|e| Error::Db {
                    error: e,
                    instance: instance.to_owned(),
                })?;
        if let Some(test) = existing {
            if interface.check_update(&test) {
                let updated: Option<Interface> = self
                    .db
                    .update(("interface", interface.get_uuid().clone().unwrap()))
                    .content(interface)
                    .await
                    .map_err(|e| Error::Db {
                        error: e,
                        instance: instance.to_owned(),
                    })?;
                match updated {
                    Some(_) => return Ok(()),
                    None => {
                        return Err(Error::UpdateFailed {
                            instance: instance.to_owned(),
                        })
                    }
                }
            } else {
                return Err(Error::UpdateUuidNotAllowed {
                    instance: instance.to_owned(),
                });
            }
        } else {
            return Err(Error::UpdateInterfaceNotExists {
                instance: instance.to_owned(),
            });
        };
    }
}
