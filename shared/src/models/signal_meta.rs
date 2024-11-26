use serde::{Deserialize, Serialize};
use surrealdb::opt::Resource::RecordId;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignalMeta {
    pub name: String,
    pub uuid: Option<String>,
    pub uom: String,
    pub uom_symbol: String,
}

impl SignalMeta {
    pub fn add_uuid(&mut self) {
        self.uuid = Some(Uuid::new_v4().to_string());
    }
    pub fn get_uuid(&self) -> String {
        self.uuid.clone().unwrap()
    }
}

impl From<SignalMeta> for String {
    fn from(value: SignalMeta) -> Self {
        value.uuid.unwrap()
    }
}
