use crate::models::shelly_v1::IsSignalResponse;
use hdc_shared::models::{
    ingestion_container::IngestionPacket, signal_data::MultiStatusData, tasklist::CollectorTask,
};
use log::{error, info, warn};
use reqwest::{self, Body, Error, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;

pub async fn collect<S>(
    task: CollectorTask,
    ingestion_url: &str,
    sender_channel: Sender<IngestionPacket>,
) -> Result<(), Error>
where
    S: DeserializeOwned + IsSignalResponse,
{
    let client = reqwest::Client::new();
    let body = reqwest::get(&task.url).await?.json::<S>().await?;

    let ingestion_body: IngestionPacket = body.to_ingestion_packet(task.signals);
    let response = client
        .post(ingestion_url)
        .body(serde_json::to_string(&ingestion_body).unwrap())
        .send()
        .await;
    match response {
        Ok(response) => handle_response(response, sender_channel, ingestion_body).await,
        Err(_) => {
            warn!("Backend not reachable, sendign data to buffer");
            buffer_data(ingestion_body, sender_channel).await
        }
    };

    Ok(())
}

async fn handle_response(
    response: Response,
    sender: Sender<IngestionPacket>,
    fallbackdata: IngestionPacket,
) {
    let response_body = response.text().await.unwrap_or_default();
    if (response_body == "Success".to_string()) {
        info!(
            "Sucessfully ingested data: {}",
            fallbackdata.data[0].timestamp
        )
    } else {
        warn!("Ingestion partially failed, sending failed data to buffer.");
        let body_bytes = response_body.as_bytes();
        let body_details = serde_json::from_slice::<serde_json::Value>(body_bytes).unwrap();
        let body_data: MultiStatusData = serde_json::from_value(body_details.get("detail").unwrap().clone()).unwrap();
        let data = IngestionPacket {
            data: body_data.failed,
        };
        buffer_data(data, sender).await;
    }
}

async fn buffer_data(data: IngestionPacket, sender: Sender<IngestionPacket>) {
    let mut retry_count: u64 = 0;
    loop {
        let buffer_response = sender.send(data.clone());
        match buffer_response.await {
            Ok(_) => {
                info!("Sent failed ingestiondata to the buffer.");
                break;
            }
            Err(e) => {
                if retry_count < 5 {
                    retry_count += 1;
                    warn!("{}", e);
                    warn!("Can't ingest data into buffer, retrying in 10s");
                    std::thread::sleep(Duration::from_secs(10));
                } else {
                    error!("Retried for 5 times, aborting buffering");
                    break;
                }
            }
        }
    }
}
