use super::error::Error;
use std::ops::Bound;

use crate::sdb::SDBRepository;
use hdc_shared::models::ingestion_container::*;
use hdc_shared::models::signal_data::*;
use hdc_shared::models::signal_meta::SignalMeta;
use hdc_shared::models::interface::Interface;

impl SDBRepository {
    pub async fn get_weather_meta(
        &self,
        ip: String,
        instance: &str,
    ) -> Result<Interface, Error> {
        let mut meta = self
            .db
            .query(format!(
                "SELECT * FROM interface WHERE url = '{}'",
                ip
            ))
            .await
            .map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;

        let mut result: Vec<Interface> = meta.take(0).map_err(|e| Error::Db {
            error: e,
            instance: instance.to_owned(),
        })?;
        Ok(result.pop().unwrap())
    }
}
