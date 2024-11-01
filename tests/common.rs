use pkarr::Keypair;
use std::sync::Arc;
use once_cell::sync::Lazy;
use pubky::PubkyClient;

pub static TEST_CLIENT: Lazy<Arc<PubkyClient>> = Lazy::new(|| {
    Arc::new(PubkyClient::default())
});

pub const HOMESERVER: &str = "ufibwbmed6jeq9k4p583go95wofakh9fwpp4k734trq79pd9u1uy";

pub static SHARED_KEYPAIR: Lazy<Keypair> = Lazy::new(Keypair::random);

pub fn generate_test_keypair() -> Keypair {
    Keypair::random()
}

pub fn get_test_setup() -> (Keypair, String, String) {
    let keypair = SHARED_KEYPAIR.clone();
    let secret_key = hex::encode(keypair.secret_key());
    let homeserver = HOMESERVER.to_string();
    (keypair, secret_key, homeserver)
}