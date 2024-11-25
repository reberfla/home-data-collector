use super::error::Error;
use std::ops::Bound;

use crate::sdb::SDBRepository;
use hdc_shared::models::ingestion_container::*;
use hdc_shared::models::signal_data::*;
use hdc_shared::models::signal_meta::SignalMeta;

impl SDBRepository {
    pub async fn ingest_data(&self, data: IngestionPacket) -> IngestionResponse {
        let mut data_it = data.data.into_iter();
        let mut success_data: Vec<Measurement> = Vec::new();
        let mut failed_data: Vec<Measurement> = Vec::new();
        let mut already_ingested: Vec<Measurement> = Vec::new();
        while let Some(dp) = data_it.next() {
            let ingest_response: Result<Option<DataPoint>, surrealdb::Error> = self
                .db
                .create((dp.uuid.clone(), dp.timestamp.clone()))
                .content(DataPoint::from(&dp))
                .await;
            match ingest_response {
                Ok(p) => success_data.push(dp),
                Err(msg) => {
                    if msg.to_string().ends_with("already exists") {
                        already_ingested.push(dp);
                    } else {
                        failed_data.push(dp);
                    }
                }
            };
        }
        if failed_data.is_empty() && already_ingested.is_empty() {
            IngestionResponse::Success
        } else {
            IngestionResponse::MultiStatus(MultiStatusData {
                success: success_data,
                failed: failed_data,
                already_exists: already_ingested,
            })
        }
    }

    pub async fn query_timeseries(
        &self,
        data: QueryTimeseriesData,
        instance: &str,
    ) -> Result<QueryResult, Error> {
        let mut response_data: Vec<SignalData> = Vec::new();
        let mut not_found: Vec<String> = Vec::new();
        let mut query = data.signals.into_iter();

        while let Some(signal) = query.next() {
            let signal_query: Result<Option<SignalMeta>, surrealdb::Error> =
                self.db.select(("signal", &signal)).await;
            let signal_response = match signal_query {
                Ok(response) => response.unwrap(),
                Err(_) => {
                    return Err(Error::QueryFailed {
                        instance: instance.to_string(),
                    })
                }
            };
            let ts_query: Result<Vec<DataPoint>, surrealdb::Error> = self
                .db
                .select(&signal)
                .range((
                    Bound::Included(data.time_from),
                    Bound::Included(data.time_to),
                ))
                .await;

            match ts_query {
                Ok(result) => {
                    let response = SignalData {
                        signal_uuid: signal_response.uuid.unwrap(),
                        signal_name: signal_response.name,
                        uom: signal_response.uom,
                        display_uom: signal_response.uom_symbol,
                        data: result,
                    };

                    response_data.push(response);
                }
                Err(_) => {
                    return Err(Error::QueryFailed {
                        instance: instance.to_string(),
                    })
                }
            }
        }
        let query_result = QueryResult {
            data: response_data,
        };
        Ok(query_result)
    }
}
