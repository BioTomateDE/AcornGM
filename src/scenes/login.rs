mod login1;

use base64::Engine;
use log::warn;
use rand::RngCore;
pub use login1::MsgLogin;

#[derive(Default, Debug, Clone)]
pub struct SceneLogin {
    pub temp_login_token: String,
    pub url: String,
}


pub fn generate_token() -> String {
    let mut buf = [0u8; 42];
    if let Err(e) = rand::rngs::OsRng.try_fill_bytes(&mut buf) {
        warn!("Could not generate cryptographically secure random bytes for token: {e}");
        rand::thread_rng().fill_bytes(&mut buf);
    };
    let generated_token: String = base64::prelude::BASE64_URL_SAFE.encode(buf);
    generated_token
}

