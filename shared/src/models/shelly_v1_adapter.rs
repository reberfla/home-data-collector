use super::signal_meta::*;
use crate::models::interface::IsAdapter;
use serde::{Deserialize, Serialize};
use std::io::{self, Error};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShellyV1Adapter {
    pub emeter_1: SignalMeta,
    pub emeter_2: SignalMeta,
    pub emeter_3: SignalMeta,
    pub emeter_4: SignalMeta,
}

impl IsAdapter for ShellyV1Adapter {
    fn add_uuid(&mut self) {
        self.emeter_1.add_uuid();
        self.emeter_2.add_uuid();
        self.emeter_3.add_uuid();
        self.emeter_4.add_uuid();
    }
    fn get_signals(&self) -> Vec<SignalMeta> {
        let mut signal: Vec<SignalMeta> = Vec::new();
        signal.push(self.emeter_1.clone());
        signal.push(self.emeter_2.clone());
        signal.push(self.emeter_3.clone());
        signal.push(self.emeter_4.clone());
        signal
    }
}
