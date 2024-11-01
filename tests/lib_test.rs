use std::string::ToString;
use serde_json;
use pubkyapp::*;

mod common;
use crate::common::*;

#[cfg(test)]
mod tests {
    use super::*;
    mod test_suite {
        use super::*;

        #[test]
        fn a_test_user_creation() {
            let (keypair, secret_key, homeserver) = get_test_setup();

            let test_user = PubkyAppUser {
                name: "Test User".to_string(),
                bio: Some("Test Bio".to_string()),
                image: None,
                links: Some(vec![UserLink {
                    title: "Test Link".to_string(),
                    url: "https://example.com".to_string(),
                }]),
                status: Some("Testing".to_string()),
            };

            let create_result = create_user(secret_key, test_user, homeserver);
            assert_eq!(create_result[0], "success", "Failed to create user: {}", create_result[1]);
        }

        #[test]
        fn b_test_sign_up_and_in() {
            let (_, secret_key, homeserver) = get_test_setup();

            let sign_in_result = sign_in(secret_key);
            assert_eq!(sign_in_result[0], "success", "Sign in failed");
        }

        #[test]
        fn c_test_put_and_get_and_list() {
            let (keypair, _, _) = get_test_setup();

            let public_key = keypair.public_key().to_string();
            let url = format!("pubky://{}/pub/test.com/testfile", public_key);
            let content = "test content".to_string();

            let inner_url = url.clone();

            let put_result = put(url.clone(), content.clone());
            assert_eq!(put_result[0], "success", "Put operation failed");

            std::thread::sleep(std::time::Duration::from_secs(1));

            let get_result = get(url);
            assert_eq!(get_result[0], "success", "Get operation failed");
            assert_eq!(get_result[1], content, "Content mismatch");

            let list_result = list(inner_url);
            assert_eq!(list_result[0], "success", "List operation failed");

            let json: serde_json::Value = serde_json::from_str(&list_result[1]).unwrap();
            assert!(json.is_array());

            if let Some(url_str) = json.as_array().and_then(|arr| arr.get(0)).and_then(|v| v.as_str()) {
                assert!(url_str.contains(&public_key));
            } else {
                panic!("Expected array with URL string");
            }
        }

        #[test]
        fn d_test_post_and_interactions() {
            let (keypair, _, _) = get_test_setup();
            let pubky = keypair.public_key().to_string();

            // Create post
            let test_post = PubkyAppPost {
                content: "Test post content".to_string(),
                kind: PostKind::Short,
                parent: None,
                embed: None,
                attachments: None,
            };

            let create_result = create_post(pubky.clone(), test_post);
            assert_eq!(create_result[0], "success", "Failed to create post");
            let post_id = create_result[1].clone();

            // Test tag operations
            let tag_result = create_tag(pubky.clone(), "test-tag".to_string(), post_id.clone());
            assert_eq!(tag_result[0], "success", "Failed to create tag");

            let tag_url = tag_result[1].clone();
            let delete_tag_result = delete_tag(tag_url);
            assert_eq!(delete_tag_result[0], "success", "Failed to delete tag");

            // Test bookmark operations
            let bookmark_result = create_bookmark(pubky.clone(), post_id.clone());
            assert_eq!(bookmark_result[0], "success", "Failed to create bookmark");

            let bookmark_url = bookmark_result[1].clone();
            let delete_bookmark_result = delete_bookmark(bookmark_url);
            assert_eq!(delete_bookmark_result[0], "success", "Failed to delete bookmark");

            // Cleanup
            let cleanup_result = cleanup_post(pubky, post_id);
            assert_eq!(cleanup_result[0], "success", "Failed to cleanup post");
        }

        #[test]
        fn e_test_follow_operations() {
            let (keypair1, _, _) = get_test_setup();
            let follower_pubky = keypair1.public_key().to_string();
            let keypair2 = generate_test_keypair();
            let followee_pubky = keypair2.public_key().to_string();

            let follow_result = create_follow(follower_pubky.clone(), followee_pubky);
            assert_eq!(follow_result[0], "success", "Failed to create follow");

            let delete_result = delete_follow(follow_result[1].clone());
            assert_eq!(delete_result[0], "success", "Failed to delete follow");
        }

        #[test]
        fn f_test_recovery_operations() {
            let (_, secret_key, _) = get_test_setup();
            let passphrase = "test_passphrase";

            let create_result = create_recovery_file(secret_key.clone(), passphrase.to_string());
            assert_eq!(create_result[0], "success", "Failed to create recovery file");

            let decrypt_result = decrypt_recovery_file(create_result[1].clone(), passphrase.to_string());
            assert_eq!(decrypt_result[0], "success", "Failed to decrypt recovery file");
            assert_eq!(decrypt_result[1], secret_key, "Decrypted key mismatch");
        }

        #[test]
        fn g_test_signout() {
            let (_, secret_key, _) = get_test_setup();

            let sign_out_result = sign_out(secret_key);
            assert_eq!(sign_out_result[0], "success", "Failed to sign out");
            assert_eq!(sign_out_result[1], "Sign out success");
        }

        #[test]
        fn h_test_error_handling() {
            let invalid_signin = sign_in("invalid_secret_key".to_string());
            assert_eq!(invalid_signin[0], "error", "Expected error for invalid sign in");

            let invalid_url = get("invalid://url".to_string());
            assert_eq!(invalid_url[0], "error", "Expected error for invalid URL");

            let invalid_recovery = create_recovery_file("".to_string(), "passphrase".to_string());
            assert_eq!(invalid_recovery[0], "error", "Expected error for empty recovery file");
        }
    }
}