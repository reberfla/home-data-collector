use super::signal_meta::*;
use crate::models::interface::IsAdapter;
use serde::{Deserialize, Serialize};
use std::io::{self, Error};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShellyV2Adapter {
    pub temp_100: SignalMeta,
    pub temp_101: SignalMeta,
    pub temp_102: SignalMeta,
    pub temp_103: SignalMeta,
}

impl IsAdapter for ShellyV2Adapter {
    fn add_uuid(&mut self) {
        self.temp_100.add_uuid();
        self.temp_101.add_uuid();
        self.temp_102.add_uuid();
        self.temp_103.add_uuid();
    }
    fn get_signals(&self) -> Vec<SignalMeta> {
        let mut signal: Vec<SignalMeta> = Vec::new();
        signal.push(self.temp_100.clone());
        signal.push(self.temp_101.clone());
        signal.push(self.temp_102.clone());
        signal.push(self.temp_103.clone());
        signal
    }
}
