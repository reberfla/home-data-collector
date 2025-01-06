use super::shelly_v1_adapter::ShellyV1Adapter;
use super::shelly_v2_adapter::ShellyV2Adapter;
use super::shelly_v1_adapter_light::ShellyV1AdapterLight;
use super::shelly_v2_adapter_light::ShellyV2AdapterLight;
use super::signal_meta::SignalMeta;
use super::weather_adapter::WeatherAdapter;
use super::tasklist::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::mem::discriminant;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Adapter {
    ShellyV1(ShellyV1Adapter),
    ShellyV2(ShellyV2Adapter),
    WeatherIngestion(WeatherAdapter),
}

impl Adapter {
    pub fn add_uuid(&mut self) {
        match self {
            Self::ShellyV1(value) => value.add_uuid(),
            Self::ShellyV2(value) => value.add_uuid(),
            Self::WeatherIngestion(value) => value.add_uuid(),
        }
    }
    pub fn get_signals(&self) -> Vec<SignalMeta> {
        match self {
            Self::ShellyV1(value) => value.get_signals(),
            Self::ShellyV2(value) => value.get_signals(),
            Self::WeatherIngestion(value) => value.get_signals(),
        }
    }
}

impl Interface {
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    pub fn get_signals(&self) -> Vec<SignalMeta> {
        self.signals.get_signals()
    }
    pub fn get_uuid(&self) -> Option<String> {
        self.uuid.clone()
    }
    pub fn check_update(&self, new_value: &Self) -> bool {
        if self.get_uuid() == new_value.get_uuid() {
            let existing_signals = self.get_signals();
            let update_signals = new_value.get_signals();
            let success: Option<()> = existing_signals
                .iter()
                .zip(update_signals.iter())
                .try_for_each(|(existing, update)| {
                    let existing_uuid = existing.get_uuid();
                    let update_uuid = update.get_uuid();
                    if existing_uuid == update_uuid {
                        Some(())
                    } else {
                        None
                    }
                });
            match success {
                Some(()) => return true,
                None => return false,
            };
        } else {
            return false;
        }
    }
    pub fn to_task(self) -> Option<CollectorTask> {
        let url: String = self.get_url();
        let adapter: Option<TaskType> = match self.signals {
            Adapter::ShellyV1(model) => {
                Some(TaskType::ShellyV1Task(ShellyV1AdapterLight::from(model)))
            }
            Adapter::ShellyV2(model) => {
                Some(TaskType::ShellyV2Task(ShellyV2AdapterLight::from(model)))
            }
            Adapter::WeatherIngestion(_)=> None
        };

        match adapter {
            None => return None,
            Some(value) => return 
                Some(CollectorTask {
                    url,
                    signals: value,
                })
        }
    }
}

pub trait IsAdapter {
    fn add_uuid(&mut self);
    fn get_signals(&self) -> Vec<SignalMeta>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Interface
{
    pub uuid: Option<String>,
    pub name: String,
    pub url: String,
    pub interface_type: String,
    pub signals: Adapter,
}

impl Interface {
    pub fn add_uuid(&mut self) {
        self.uuid = Some(Uuid::new_v4().to_string());
        self.signals.add_uuid()
    }
    pub fn get_global_id(&self) -> &Option<String> {
        &self.uuid
    }
}
