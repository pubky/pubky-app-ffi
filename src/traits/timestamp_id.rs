use axum::async_trait;
use pubky_common::timestamp::Timestamp;
use base32::{decode, Alphabet};
use chrono::{DateTime, Duration, NaiveDate, Utc};

#[async_trait]
pub trait TimestampId {
    /// Creates a unique identifier based on the current timestamp.
    fn create_id(&self) -> String {
        let timestamp = Timestamp::now();
        timestamp.to_string()
    }

    /// Validates that the provided ID is a valid Crockford Base32-encoded timestamp,
    /// 13 characters long, and represents a reasonable timestamp.
    async fn validate_id(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Ensure ID is 13 characters long
        if id.len() != 13 {
            return Err("Invalid ID length: must be 13 characters".into());
        }

        // Decode the Crockford Base32-encoded ID
        let decoded_bytes =
            decode(Alphabet::Crockford, id).ok_or("Failed to decode Crockford Base32 ID")?;

        if decoded_bytes.len() != 8 {
            return Err("Invalid ID length after decoding".into());
        }

        // Convert the decoded bytes to a timestamp in microseconds
        let timestamp_micros = i64::from_be_bytes(decoded_bytes.try_into().unwrap_or_default());
        let timestamp: i64 = timestamp_micros / 1_000_000;

        // Convert the timestamp to a DateTime<Utc>
        let id_datetime = DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_default()
            .date_naive();

        // Define October 1st, 2024, at 00:00:00 UTC
        let oct_first_2024 = NaiveDate::from_ymd_opt(2024, 10, 1).expect("Invalid date");

        // Allowable future duration (2 hours)
        let max_future = Utc::now().date_naive() + Duration::hours(2);

        // Validate that the ID's timestamp is after October 1st, 2024
        if id_datetime < oct_first_2024 {
            return Err("Invalid ID: timestamp must be after October 1st, 2024".into());
        }

        // Validate that the ID's timestamp is not more than 2 hours in the future
        if id_datetime > max_future {
            return Err("Invalid ID: timestamp is too far in the future".into());
        }

        Ok(())
    }
}