#![allow(unused)]
use config::CollectorConfig;
use log::{debug, info, warn};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use std::fs;
use tokio::sync::{mpsc::channel, OnceCell};
use tokio::time::{interval_at, Duration, Instant};

use hdc_shared::models::ingestion_container::IngestionPacket;
use hdc_shared::models::tasklist::Tasklist;
mod buffer;
mod collector;
mod config;
mod models;
mod task;

use buffer::{buffer_handler, buffer_ingestor};
use task::{task_dispatcher, tasklist_observer};

static TASKLIST: RwLock<Tasklist> = RwLock::new(Tasklist::new());

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Arc<CollectorConfig> = Arc::new(CollectorConfig::load("./Config.yml"));
    std::env::set_var("RUST_LOG", &config.loglevel);
    env_logger::init();
    debug!("{:?}", config);
    fs::create_dir_all(&config.buffer_path);
    let (send, mut recv) = channel::<IngestionPacket>(32);

    tasklist_observer(&TASKLIST, &config.tasklist_url).await;

    let sys_time_now = SystemTime::now();
    let start_offset = sys_time_now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % &config.collection_interval;

    let start_collecting = Instant::now()
        + Duration::from_secs(
            (&config.collection_interval - start_offset)
                .try_into()
                .unwrap(),
        );

    let tasklist_observer_config = config.clone();
    tokio::spawn(async move {
        let mut interval = interval_at(
            start_collecting + Duration::from_secs(5),
            Duration::from_secs(tasklist_observer_config.task_update_interval),
        );
        loop {
            interval.tick().await;
            let tasklist_status =
                tasklist_observer(&TASKLIST, &tasklist_observer_config.tasklist_url).await;
            match tasklist_status {
                Ok(()) => info!("Tasklist updated"),
                Err(e) => warn!("Failed to get latest Tasklist: {:?}", e),
            }
        }
    });

    let collector_config = config.clone();
    let path = format!("{}", &collector_config.buffer_path);
    let ingestion_url = Arc::new(collector_config.ingestion_url.clone());
    tokio::spawn(async move {
        let mut interval = interval_at(
            start_collecting,
            Duration::from_secs(collector_config.collection_interval),
        );
        loop {
            interval.tick().await;
            task_dispatcher(&TASKLIST, ingestion_url.clone(), send.clone()).await;
        }
    });
    tokio::spawn(async move {
        buffer_handler(&path, recv).await;
    });

    let buffer_config = config.clone();
    tokio::spawn(async move {
        let mut interval = interval_at(
            start_collecting + Duration::from_secs(15),
            Duration::from_secs(buffer_config.buffer_ingestion_interval),
        );
        loop {
            interval.tick().await;
            let status =
                buffer_ingestor(&buffer_config.buffer_path, &buffer_config.ingestion_url).await;
            match status {
                Ok(()) => (),
                Err(e) => warn!("bufer_ingestor: {:?}", e),
            }
        }
    });

    loop {}
}
