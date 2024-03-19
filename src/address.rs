use crate::{Coordinates, Fields};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressComponents {
    pub number: Option<String>,
    pub street: Option<String>,
    pub suffix: Option<String>,
    #[serde(rename = "secondarynumber")]
    pub secondary_number: Option<String>,
    #[serde(rename = "secondaryunit")]
    pub secondary_unit: Option<String>,
    #[serde(rename = "postdirectional")]
    pub post_directional: Option<String>,
    pub formatted_street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub county: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "predirectional")]
    pub pre_directional: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub query: Option<String>,
    pub address_components: AddressComponents,
    pub formatted_address: String,
    pub location: Coordinates,
    pub accuracy: f64,
    pub accuracy_type: String,
    pub source: String,
    pub fields: Option<Fields>,
}
