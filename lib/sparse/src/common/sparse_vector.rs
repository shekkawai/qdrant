use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::types::{DimId, DimWeight};

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct SparseVector {
    pub indices: Vec<DimId>,
    pub weights: Vec<DimWeight>,
}

impl SparseVector {
    pub fn new(indices: Vec<DimId>, weights: Vec<DimWeight>) -> SparseVector {
        SparseVector { indices, weights }
    }
}
