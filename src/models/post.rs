use serde::{Deserialize, Serialize};
use uniffi;
use uuid::Uuid;

#[derive(uniffi::Enum, Serialize, Deserialize, Clone)]
pub enum PostKind {
    Short,
    Long,
    Image,
    Video,
    Link,
    File,
}

#[derive(uniffi::Record, Serialize, Deserialize, Clone)]
pub struct PostEmbed {
    pub kind: PostKind,
    pub uri: String,
}

#[derive(uniffi::Record, Serialize, Deserialize, Clone)]
pub struct PubkyAppPost {
    pub content: String,
    pub kind: PostKind,
    pub parent: Option<String>,
    pub embed: Option<PostEmbed>,
    pub attachments: Option<Vec<String>>,
}

impl PubkyAppPost {
    pub fn create_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}