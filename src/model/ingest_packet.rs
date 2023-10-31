use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value, Thing};
use surrealdb::opt::RecordId;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestionPacket {
    pub data : Vec<DataPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPoint {
    pub timestamp: i64,
    pub suuid: String,
    pub value: f64,
}


impl DataPoint {
    fn new(ts: i64,signal_uuid: String, measured_value: f64) -> Self {
        DataPoint{
            timestamp: ts,
            suuid: signal_uuid,
            value: measured_value
        }
    }
}

impl IngestionPacket {
    pub fn new(datapoints: Vec<DataPoint>) -> Self {
        IngestionPacket{
            data: datapoints
        }
    }
}
