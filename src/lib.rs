mod address;
mod congressional;
mod errors;
mod types;
pub use address::*;
pub use congressional::*;
pub use errors::Error;
use serde::{Deserialize, Serialize};
pub use types::*;

const GEOCODIO_BASE_URL: &str = "https://api.geocod.io/v1.7/";

pub struct GeocodioProxy {
    client: reqwest::Client,
    pub base_url: reqwest::Url,
    api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    address_components: Components,
    formatted_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeResponse {
    input: Input,
    results: Vec<Address>,
    debug: Option<Debug>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debug {}

#[derive(Serialize, Deserialize)]
pub struct AddressInput {
    pub line_1: String,
    pub line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}

pub enum AddressParams {
    String(String),
    AddressInput(AddressInput),
}

impl GeocodioProxy {
    /// Instantiate new GeocodioProxy API client from .env GEOCODIO_API_KEY variable
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let api_key = std::env::var("GEOCODIO_API_KEY")?;
        let client = reqwest::Client::new();

        Ok(GeocodioProxy {
            client,
            base_url: reqwest::Url::parse(GEOCODIO_BASE_URL).unwrap(),
            api_key,
        })
    }

    /// Instantiate new GeocodioProxy API client by passing api key
    pub fn new_from_key(api_key: String) -> Result<Self, Error> {
        let client = reqwest::Client::new();

        Ok(GeocodioProxy {
            client,
            base_url: reqwest::Url::parse(GEOCODIO_BASE_URL).unwrap(),
            api_key,
        })
    }

    /// Helper function to wrap the request
    pub async fn request(&self, endpoint: &str, params: &str) -> Result<reqwest::Response, Error> {
        let mut url = self.base_url.join(endpoint).unwrap();
        url.set_query(Some(params));
        url.query_pairs_mut().append_pair("api_key", &self.api_key);
        Ok(self.client.get(url).send().await.unwrap())
    }

    /// Geocode a single address
    pub async fn geocode(
        &self,
        address: AddressParams,
        fields: Option<&[&str]>,
    ) -> Result<GeocodeResponse, Error> {
        let mut params = match address {
            AddressParams::String(address) => address.to_string(),
            AddressParams::AddressInput(address) => format!(
                "street={}&city={}&state={}&country={}&postal_code={}",
                address.line_1, address.city, address.state, address.country, address.postal_code
            ),
        };
        if let Some(fields) = fields {
            params.push_str(format!("&fields={}", fields.join(",")).as_str());
        }
        let response = self.request("geocode", &params).await?;
        let json = &response.json::<serde_json::Value>().await.unwrap();
        // println!("{}", serde_json::to_string_pretty(&json).unwrap());
        let result: GeocodeResponse = serde_json::from_value(json.clone()).unwrap();
        Ok(result)
    }

    /// Reverse geocode a tuple of (lat,lng)
    pub async fn reverse_geocode(
        &self,
        coordinates: (f64, f64),
    ) -> Result<reqwest::Response, Error> {
        let params = format!("lat={}&lng={}", coordinates.0, coordinates.1);
        self.request("reverse_geocode", &params).await
    }

    // TODO: batch geocode and reverse geocode
}

#[tokio::test]
async fn test_geocodio_request() {
    let geocodio = GeocodioProxy::new().unwrap();
    let response = geocodio
        .request("geocode", "q=1600+Amphitheatre+Parkway,+Mountain+View,+CA")
        .await
        .unwrap();
    // let json = response.json::<serde_json::Value>().await.unwrap();
    // println!("{}", serde_json::to_string_pretty(&json).unwrap());
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_geocode() {
    let geocodio = GeocodioProxy::new().unwrap();
    let response = geocodio
        .geocode(
            AddressParams::AddressInput(AddressInput {
                line_1: "2322 N Marion St".to_string(),
                line_2: None,
                city: "Denver".to_string(),
                state: "CO".to_string(),
                country: "US".to_string(),
                postal_code: "80205".to_string(),
            }),
            Some(&["acs-economics", "zip4"]),
        )
        .await
        .unwrap();
    // println!("{}", serde_json::to_string_pretty(&response).unwrap());
    assert!(!response.results.is_empty());
}
