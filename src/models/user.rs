use serde::{Deserialize, Serialize};
use uniffi;

#[derive(Serialize, Deserialize, Default, Clone, Debug, uniffi::Record)]
pub struct UserLink {
    pub title: String,
    pub url: String,
}

/// Profile schema
/// URI: /pub/pubky.app/profile.json
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct PubkyAppUser {
    pub name: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub links: Option<Vec<UserLink>>,
    pub status: Option<String>,
}

#[derive(uniffi::Record, Serialize, Deserialize, Default)]
pub struct PubkyAppFollow {
    pub created_at: i64,
}
