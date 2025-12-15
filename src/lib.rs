//! Cloudflare colo (data center) to Durable Objects location hint mapping.
//!
//! This library provides a mapping from Cloudflare edge locations (colos) to
//! the recommended Durable Objects location hints. This is useful when you need
//! to provide location hints for Durable Objects based on incoming request colos.
//!
//! # Example
//!
//! ```
//! use cf_colo_hint::{Colo, LocationHint};
//!
//! // Parse a colo from the cf-ray header or request metadata
//! let colo = Colo::from_code("LAX").expect("unknown colo");
//!
//! // Get the recommended location hint for Durable Objects
//! if let Some(hint) = colo.location_hint() {
//!     println!("Use location hint: {}", hint.as_str());
//!     // Use hint.as_str() when calling the Durable Objects API
//! }
//!
//! // You can also work with location hints directly
//! let hint = LocationHint::WNam;
//! assert_eq!(hint.as_str(), "wnam");
//! assert_eq!(hint.name(), "Western North America");
//! ```
//!
//! # Data Sources
//!
//! The colo data comes from:
//! - `components.json`: Cloudflare status page data with colo names and regions
//! - `where.durableobjects.live.json`: Measured latency data for DO region mapping
//!
//! Run `python3 codegen.py` to regenerate the Rust code from updated JSON files.

#![no_std]

mod generated;

pub use generated::{Colo, LocationHint};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_hint_roundtrip() {
        for hint in LocationHint::ALL {
            let code = hint.as_str();
            let parsed = LocationHint::parse(code);
            assert_eq!(parsed, Some(*hint));
        }
    }

    #[test]
    fn test_colo_roundtrip() {
        for colo in Colo::ALL {
            let code = colo.code();
            let parsed = Colo::from_code(code);
            assert_eq!(parsed, Some(*colo));
        }
    }

    #[test]
    fn test_known_colos() {
        // Test some well-known colos
        let lax = Colo::from_code("LAX").unwrap();
        assert_eq!(lax.name(), "Los Angeles, CA, United States");
        assert_eq!(lax.location_hint(), Some(LocationHint::WNam));

        let ams = Colo::from_code("AMS").unwrap();
        assert_eq!(ams.name(), "Amsterdam, Netherlands");
        // AMS may map to afr, weur, or eeur depending on measured latency

        let sin = Colo::from_code("SIN").unwrap();
        assert!(sin.name().contains("Singapore"));
        assert_eq!(sin.location_hint(), Some(LocationHint::Apac));
    }

    #[test]
    fn test_location_hint_names() {
        assert_eq!(LocationHint::WNam.name(), "Western North America");
        assert_eq!(LocationHint::ENam.name(), "Eastern North America");
        assert_eq!(LocationHint::WEur.name(), "Western Europe");
        assert_eq!(LocationHint::EEur.name(), "Eastern Europe");
        assert_eq!(LocationHint::Apac.name(), "Asia-Pacific");
        assert_eq!(LocationHint::Oc.name(), "Oceania");
        assert_eq!(LocationHint::Sam.name(), "South America");
        assert_eq!(LocationHint::Afr.name(), "Africa");
        assert_eq!(LocationHint::Me.name(), "Middle East");
    }

    #[test]
    fn test_display() {
        // Display impl returns the code
        assert_eq!(LocationHint::WNam.as_str(), "wnam");
        assert_eq!(Colo::LAX.code(), "LAX");
    }
}
