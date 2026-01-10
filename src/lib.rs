mod address;
mod congressional;
mod errors;
mod types;
mod utils;
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    address_components: AddressComponents,
    formatted_address: String,
}

// Single Response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeResponse {
    pub input: Input,
    pub results: Vec<Address>,
    pub debug: Option<Debug>,
}

// Batch Response
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeocodeBatchResponse {
    // pub input: Input,
    pub results: Option<Vec<BatchResult>>,
    pub debug: Option<Debug>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchResult {
    pub query: Option<String>,
    pub response: Option<Response>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub input: Option<Input>,
    pub results: Option<Vec<ResponseResult>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseResult {
    pub address_components: Option<AddressComponents>,
    pub formatted_address: Option<String>,
    pub location: Option<Location>,
    pub accuracy: Option<f64>,
    pub accuracy_type: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Debug {}

#[derive(Serialize, Deserialize)]
pub struct AddressInput {
    pub line_1: Option<String>,
    pub line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
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
            AddressParams::AddressInput(address) => address.fmt_string(),
        };
        if let Some(fields) = fields {
            params.push_str(format!("&fields={}", fields.join(",")).as_str());
        }
        let response = self.request("geocode", &params).await?;
        let json = &response.json::<serde_json::Value>().await.unwrap();
        // println!("{}", serde_json::to_string_pretty(&json).unwrap());
        let result = serde_json::from_value(json.clone());

        match result {
            Ok(geocode_response) => Ok(geocode_response),
            Err(e) => Err(Error::BadAddress(e)),
        }
    }

    /// Reverse geocode a tuple of (lat,lng)
    pub async fn reverse_geocode(
        &self,
        coordinates: (f64, f64),
    ) -> Result<reqwest::Response, Error> {
        let params = format!("lat={}&lng={}", coordinates.0, coordinates.1);
        self.request("reverse_geocode", &params).await
    }

    // Request Batch
    pub async fn request_batch(&self, endpoint: &str, params: Vec<String>) -> Result<reqwest::Response, Error> {
        let url = self.base_url.join(endpoint).unwrap();
        let mut payload: Vec<String> = Vec::new();

        params.iter().enumerate().for_each(|(_i, address)| {
            payload.push(serde_json::Value::String(address.to_owned()).to_string());
        });
        let res = self.client.post(url).json(&payload).send().await?;
        Ok(res)
    }

    // Batch Geocode
    pub async fn geocode_batch(&self, addresses: Vec<AddressParams>) -> Result<GeocodeBatchResponse, Error> {
        let mut params: Vec<String> = Vec::new();
        addresses.iter().for_each(|address| {
            match address {
                AddressParams::String(address) => params.push(address.to_string()),
                AddressParams::AddressInput(address) => params.push(address.to_string()),
            };
        });
        let endpoint = format!("geocode?api_key={}", &self.api_key);
        let res = self.request_batch(endpoint.as_str(), params).await?;
        let json = &res.json::<serde_json::Value>().await?;
        let result = serde_json::from_value::<GeocodeBatchResponse>(json.clone());
        match result {
            Ok(geocode_response) => Ok(geocode_response),
            Err(e) => Err(Error::BadAddress(e)),
        }
    }

    // TODO: reverse geocode
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
                line_1: Some("48965 Co Rd 262".to_string()),
                line_2: None,
                city: Some("Marcell".to_string()),
                state: Some("MN".to_string()),
                country: Some("US".to_string()),
                postal_code: Some("56657".to_string()),
            }),
            Some(&["cd", "stateleg"]),
        )
        .await
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    assert!(!response.results.is_empty());
}

#[tokio::test]
async fn test_geocode_batch() {
    let geocodio = GeocodioProxy::new().unwrap();

    let addresses: Vec<AddressParams> = vec![
        AddressParams::String("1109 N Highland St, Arlington VA".to_string()), 
        AddressParams::String("525 University Ave, Toronto, ON, Canada".to_string()), 
        AddressParams::String("4410 S Highway 17 92, Casselberry FL".to_string()), 
        AddressParams::String("15000 NE 24th Street, Redmond WA".to_string()), 
        AddressParams::String("17015 Walnut Grove Drive, Morgan Hill CA".to_string())
    ];

    let response = geocodio
        .geocode_batch(addresses)
        .await
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    assert!(!response.results.is_none());
}