use axum::async_trait;
use serde::de::DeserializeOwned;
use url::Url;
use crate::{HashId, PubkyAppTag, MAX_TAG_LABEL_LENGTH};
use axum::body::Bytes;

#[async_trait]
impl Validatable for PubkyAppTag {
    async fn sanitize(self) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Convert label to lowercase and trim
        let label = self.label.trim().to_lowercase();

        // Enforce maximum label length
        let label = if label.len() > MAX_TAG_LABEL_LENGTH {
            label[..MAX_TAG_LABEL_LENGTH].to_string()
        } else {
            label
        };

        // Sanitize URI
        let uri = match Url::parse(&self.uri) {
            Ok(url) => url.to_string(),
            Err(_) => return Err("Invalid URI in tag".into()),
        };

        Ok(PubkyAppTag {
            uri,
            label,
            created_at: self.created_at,
        })
    }

    async fn validate(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.validate_id(id).await?;

        // Validate label length
        if self.label.len() > MAX_TAG_LABEL_LENGTH {
            return Err("Tag label exceeds maximum length".into());
        }

        // TODO: more validation?

        Ok(())
    }
}

#[async_trait]
pub trait Validatable: Sized + DeserializeOwned {
    async fn try_from(
        blob: &Bytes,
        id: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut instance: Self = serde_json::from_slice(blob)?;
        instance = instance.sanitize().await?;
        instance.validate(id).await?;
        Ok(instance)
    }

    async fn validate(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn sanitize(self) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self)
    }
}