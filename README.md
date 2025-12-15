# cf-colo-hint

A Rust library that maps Cloudflare edge locations (colos) to Durable Objects location hints.

## Usage

```rust
use cf_colo_hint::{Colo, LocationHint};

// Parse a colo from the cf-ray header or request metadata
let colo = Colo::from_code("LAX").expect("unknown colo");

// Get the recommended location hint for Durable Objects
if let Some(hint) = colo.location_hint() {
    println!("Use location hint: {}", hint.as_str());
}

// Work with location hints directly
let hint = LocationHint::WNam;
assert_eq!(hint.as_str(), "wnam");
assert_eq!(hint.name(), "Western North America");
```

## Features

- `no_std` compatible
- Zero dependencies
- Auto-generated from Cloudflare's status page and measured latency data

## Data Sources

Colo data is sourced from:

- [Cloudflare Status](https://www.cloudflarestatus.com/) - colo names and regions
- [where.durableobjects.live](https://where.durableobjects.live/) - measured latency data for DO region mapping

## Updating Data

Run the refresh script to pull the latest data and regenerate the Rust code:

```bash
./refresh.sh
```

## License

MIT
