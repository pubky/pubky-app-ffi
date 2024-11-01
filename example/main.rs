use std::string::ToString;
use std::sync::Arc;
use once_cell::sync::Lazy;
use pubky::PubkyClient;
use std::str;
use pubkyapp::*;

static PUBKY_CLIENT: Lazy<Arc<PubkyClient>> = Lazy::new(|| {
    // let custom_testnet = Testnet {
    //     bootstrap: vec!["http://localhost:6287".to_string()],
    //     nodes: vec![],
    // };
    //
    // let client = PubkyClient::builder()
    //     .testnet(&custom_testnet)
    //     .build();
    let client = PubkyClient::default();

    Arc::new(client)
});

const HOMESERVER: &str = "ufibwbmed6jeq9k4p583go95wofakh9fwpp4k734trq79pd9u1uy";
const SECRET_KEY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn main() {
    // Basic authentication flow
    let sign_in_res = signin_or_signup(SECRET_KEY, HOMESERVER);
    println!("Sign In/Up Response: {:?}", sign_in_res);
    if sign_in_res[0] == "error" {
        return;
    }
    let pubky = sign_in_res[1].clone();
    // Sleep for one second
    std::thread::sleep(std::time::Duration::from_secs(2));
    //
    // // Example user creation
    // let user = PubkyAppUser {
    //     name: "John Doe".to_string(),
    //     bio: Some("A test user".to_string()),
    //     image: None,
    //     links: Some(vec![UserLink {
    //         title: "Website".to_string(),
    //         url: "https://example.com".to_string(),
    //     }]),
    //     status: Some("Available".to_string()),
    // };
    //
    // let create_user_res = create_user(SECRET_KEY.to_string(), user, HOMESERVER.to_string());
    // println!("Create User Response: {:?}", create_user_res);

    // Example post creation
    let post = PubkyAppPost {
        content: "Hello, Pubky!".to_string(),
        kind: PostKind::Short,
        parent: None,
        embed: None,
        attachments: None,
    };

    let create_post_res = create_post(pubky, post);
    println!("Create Post Response: {:?}", create_post_res);

    // // Example tag creation
    // let create_tag_res = create_tag(
    //     HOMESERVER.to_string(),
    //     "example".to_string(),
    //     "post123".to_string(),
    // );
    // println!("Create Tag Response: {:?}", create_tag_res);
    //
    // // Example bookmark creation
    // let create_bookmark_res = create_bookmark(HOMESERVER.to_string(), "post123".to_string());
    // println!("Create Bookmark Response: {:?}", create_bookmark_res);
    //
    // // Example file handling
    // let file = PubkyAppFile {
    //     name: "example.txt".to_string(),
    //     created_at: chrono::Utc::now().timestamp_millis(),
    //     src: "data:text/plain;base64,SGVsbG8gV29ybGQ=".to_string(),
    //     content_type: "text/plain".to_string(),
    //     size: 11,
    // };
    //
    // let create_file_res = create_file(HOMESERVER.to_string(), &file);
    // println!("Create File Response: {:?}", create_file_res);
    //
    // // Example follow relationship
    // let create_follow_res = create_follow(
    //     HOMESERVER.to_string(),
    //     "another_user_pubky".to_string(),
    // );
    // println!("Create Follow Response: {:?}", create_follow_res);
    //
    // // Example publishing and retrieving content
    // let publish_res = publish(
    //     "example.com".to_string(),
    //     "Hello World".to_string(),
    //     SECRET_KEY.to_string(),
    // );
    // println!("Publish Response: {:?}", publish_res);
    //
    // // Example URL construction and content retrieval
    // if let "success" = publish_res[0].as_str() {
    //     let public_key = &publish_res[1];
    //     let url = construct_pubky_url(public_key, "example.com", &[]);
    //
    //     let put_res = put(url.clone(), "New content".to_string());
    //     println!("Put Response: {:?}", put_res);
    //
    //     let get_res = get(url.clone());
    //     println!("Get Response: {:?}", get_res);
    //
    //     let list_res = list(url.clone());
    //     println!("List Response: {:?}", list_res);
    // }
    //
    // // Example cleanup operations
    // let cleanup_post_res = cleanup_post(HOMESERVER.to_string(), "post123".to_string());
    // println!("Cleanup Post Response: {:?}", cleanup_post_res);
    //
    // let cleanup_user_res = cleanup_user(HOMESERVER.to_string());
    // println!("Cleanup User Response: {:?}", cleanup_user_res);
}

pub fn signin_or_signup(secret_key: &str, homeserver: &str) -> Vec<String> {
    let sign_in_res = sign_in(secret_key.to_string());
    if sign_in_res[0] == "success" {
        return sign_in_res;
    }
    sign_up(secret_key.to_string(), homeserver.to_string())
}

// Helper function to construct Pubky URLs
fn construct_pubky_url(public_key: &str, domain: &str, path_segments: &[&str]) -> String {
    let mut url = format!("pubky://{}/pub/{}", public_key, domain);

    for segment in path_segments {
        if !segment.is_empty() {
            url.push('/');
            url.push_str(segment);
        }
    }

    if url.ends_with('/') {
        url.pop();
    }

    url
}

// Helper function to create response vectors
fn create_response_vector(error: bool, data: String) -> Vec<String> {
    if error {
        vec!["error".to_string(), data]
    } else {
        vec!["success".to_string(), data]
    }
}