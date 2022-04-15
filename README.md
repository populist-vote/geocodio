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
                line_1: "Black Rock Desert".to_string(),
                line_2: None,
                city: "Gerlach".to_string(),
                state: "NV".to_string(),
                country: "US".to_string(),
                postal_code: "89412".to_string(),
            }),
            Some(&["acs-economics", "zip4"]),
        )
        .await
        .unwrap();
   println!(
        "Burning Man is located at the coordinates: ({}, {})",
        response.results[0].location.latitude, response.results[0].location.longitude
    )
}
```
