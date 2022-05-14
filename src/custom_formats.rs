// Module for deserialization of custom day format (e.g. 24.12.2004)
pub mod custom_date_format {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer};

    // Custom date format
    const FORMAT: &'static str = "%d.%m.%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Apply custom date format
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

// Module for deserialization of custom (European) float format (e.g.: 24.016,56)
pub mod custom_float_format {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Remove dots and replace commas with dots
        s.replace('.', "")
            .replace(',', ".")
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
