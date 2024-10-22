use serde::{Deserialize, Serialize};
use uniffi;

/// Represents raw homeserver tag with id
/// URI: /pub/pubky.app/tags/:tag_id
///
/// Example URI:
///
/// `/pub/pubky.app/tags/FPB0AM9S93Q3M1GFY1KV09GMQM`
///
/// Where tag_id is Crockford-base32(Blake3("{uri_tagged}:{label}")[:half])
#[derive(uniffi::Record, Serialize, Deserialize, Clone)]
pub struct PubkyAppTag {
    pub uri: String,
    pub label: String,
    pub created_at: i64,
}