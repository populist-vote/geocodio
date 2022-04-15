# Geocodio

A client library for the [Geocodio](https://www.geocod.io/) API

# Usage

## Geocode

```rust
use geocodio::GeocodioProxy;

fn main() {
    let geocodio = GeocodioProxy::new().unwrap());
    let response = geocodio
        .geocode(
            AddressParams::AddressInput(AddressInput {
                line_1: "2322 N Marion St".to_string(),
                line_2: None,
                city: "Denver".to_string(),
                state: "C0".to_string(),
                country: "US".to_string(),
                postal_code: "80205".to_string(),
            }),
            &["cd", "stateleg"],
        )
        .await
        .unwrap();
    let json = response.json::<serde_json::Value>().await.unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
```
