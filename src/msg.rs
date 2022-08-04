use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{ManufactureId, PharmacistId, SymptomToken, BatchId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    // pub pharmacists: Vec<PharmacistId>,
    // pub manufacturers: Vec<ManufactureId>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateBatch { batch_id: BatchId, locations: Vec<String>, threshold: u64},
    // store K=batch_id V = batch struct
    AddPatient { symptom_token: SymptomToken, batch_id: BatchId},
    // storage K=symptom_token||batch_id V=bool
    AddSymptom { symptom_token: SymptomToken, batch_id: BatchId}
    // check K=symptom_token
    // store K=batch_id V=increment counter
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CheckBatch {batch_id: BatchId}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CheckBatchResponse {
    pub threshold_reached: bool,
    pub locations: Vec<String>
}
