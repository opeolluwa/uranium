//! Money conversion between the minor units stored in the database (kobo) and
//! the major units exchanged with clients.

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serializer};

pub const MINOR_UNITS_PER_MAJOR: i64 = 100;

pub fn to_minor_units(major: i64) -> Option<i64> {
    major.checked_mul(MINOR_UNITS_PER_MAJOR)
}

pub fn to_major_units(minor: i64) -> Decimal {
    (Decimal::from(minor) / Decimal::from(MINOR_UNITS_PER_MAJOR)).normalize()
}

/// Renders a minor-unit amount as a major-unit string with two decimal places.
pub fn format_minor_units(minor: i64) -> String {
    format!("{:.2}", to_major_units(minor))
}

pub fn deserialize_price<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer)?
        .checked_mul(MINOR_UNITS_PER_MAJOR)
        .ok_or_else(|| serde::de::Error::custom("price overflows when converted to minor units"))
}

pub fn serialize_price<S>(price: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_major_units(*price).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    struct PriceOut {
        #[serde(serialize_with = "super::serialize_price")]
        price: i64,
    }

    #[derive(Deserialize)]
    struct PriceIn {
        #[serde(deserialize_with = "super::deserialize_price")]
        price: i64,
    }

    #[test]
    fn serializes_whole_major_units_as_string() {
        assert_eq!(
            serde_json::to_string(&PriceOut { price: 500_000 }).unwrap(),
            r#"{"price":"5000"}"#
        );
    }

    #[test]
    fn serializes_fractional_and_sub_unit_prices() {
        for (minor, major) in [(5_050, "50.5"), (1, "0.01"), (99, "0.99"), (0, "0")] {
            assert_eq!(
                serde_json::to_string(&PriceOut { price: minor }).unwrap(),
                format!(r#"{{"price":"{major}"}}"#)
            );
        }
    }

    #[test]
    fn serializes_as_json_string_not_number() {
        let value = serde_json::to_value(PriceOut { price: 12_345 }).unwrap();
        assert_eq!(value["price"], serde_json::json!("123.45"));
        assert!(value["price"].is_string());
    }

    #[test]
    fn serializes_negative_minor_units_without_panicking() {
        assert_eq!(
            serde_json::to_string(&PriceOut { price: -250 }).unwrap(),
            r#"{"price":"-2.5"}"#
        );
    }

    #[test]
    fn deserializes_major_units_into_minor_units() {
        let parsed: PriceIn = serde_json::from_str(r#"{"price":5000}"#).unwrap();
        assert_eq!(parsed.price, 500_000);
    }

    #[test]
    fn deserializes_zero() {
        let parsed: PriceIn = serde_json::from_str(r#"{"price":0}"#).unwrap();
        assert_eq!(parsed.price, 0);
    }

    #[test]
    fn rejects_overflow() {
        let result: Result<PriceIn, _> = serde_json::from_str(r#"{"price":9223372036854775807}"#);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_integer_and_string_prices() {
        let fractional: Result<PriceIn, _> = serde_json::from_str(r#"{"price":50.5}"#);
        let stringified: Result<PriceIn, _> = serde_json::from_str(r#"{"price":"5000"}"#);
        assert!(fractional.is_err());
        assert!(stringified.is_err());
    }

    #[test]
    fn round_trips_major_units_through_minor_units() {
        for major in [0, 1, 99, 5_000, 123_456] {
            let minor: PriceIn = serde_json::from_str(&format!(r#"{{"price":{major}}}"#)).unwrap();
            let out = serde_json::to_value(PriceOut { price: minor.price }).unwrap();

            assert_eq!(minor.price, to_minor_units(major).unwrap());
            assert_eq!(out["price"], serde_json::json!(major.to_string()));
        }
    }

    #[test]
    fn to_minor_units_converts_and_detects_overflow() {
        assert_eq!(to_minor_units(0), Some(0));
        assert_eq!(to_minor_units(1), Some(100));
        assert_eq!(to_minor_units(i64::MAX), None);
    }

    #[test]
    fn format_minor_units_always_keeps_two_decimals() {
        assert_eq!(format_minor_units(0), "0.00");
        assert_eq!(format_minor_units(1), "0.01");
        assert_eq!(format_minor_units(5_050), "50.50");
        assert_eq!(
            format_minor_units(9_007_199_254_740_993),
            "90071992547409.93"
        );
    }

    #[test]
    fn to_major_units_normalizes_scale() {
        assert_eq!(to_major_units(100), Decimal::from(1));
        assert_eq!(to_major_units(5_050).to_string(), "50.5");
        assert_eq!(to_major_units(1), Decimal::new(1, 2));
        assert_eq!(to_major_units(0), Decimal::from(0));
    }
}
