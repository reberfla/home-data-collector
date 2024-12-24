use super::interface::{Interface, IsAdapter};
use super::shelly_v1_adapter_light::ShellyV1AdapterLight;
use super::shelly_v2_adapter_light::ShellyV2AdapterLight;
use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasklist {
    pub tasks: Vec<CollectorTask>,
}

impl Tasklist {
    pub const fn new() -> Tasklist {
        Tasklist { tasks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectorTask {
    pub url: String,
    pub signals: TaskType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    ShellyV1Task(ShellyV1AdapterLight),
    ShellyV2Task(ShellyV2AdapterLight),
}
