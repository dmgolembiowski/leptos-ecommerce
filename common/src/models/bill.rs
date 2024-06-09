use crate::models::types::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Builder)]
#[builder]
pub struct BillRow {
    pub id: BillRowId,
    pub customer_id: CustomerId,
    pub line_items: BillLineItems,
    pub total: Money,
    pub created_at: Time,
    pub updated_at: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BillLineItems(pub BTreeMap<InventoryName, Count>);

impl std::fmt::Display for BillLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use serde_json;
        write!(f, "{}", serde_json::to_string(&self.0).unwrap())
    }
}

impl TryFrom<&str> for BillLineItems {
    type Error = serde_json::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::from_str(s)?))
    }
}

impl BillLineItems {
    pub fn new(inner: BTreeMap<InventoryName, Count>) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> BTreeMap<InventoryName, Count> {
        self.0
    }

    pub fn dumps(&self) -> String {
        self.to_string()
    }

    pub fn loads(s: &str) -> Result<Self, serde_json::Error> {
        <Self as TryFrom<&str>>::try_from(s)
    }
}

impl Iterator for BillLineItems {
    type Item = (InventoryName, Count);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().map(|(k, v)| (k.clone(), v.clone()))
    }
}
