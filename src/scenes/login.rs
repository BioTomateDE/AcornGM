mod login1;

use std::any::type_name_of_val;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use base64::Engine;
use copypasta::ClipboardContext;
use log::warn;
use rand::{RngCore, TryRngCore};
pub use login1::MsgLogin;

#[derive(Default)]
pub struct SceneLogin {
    pub temp_login_token: String,
    pub url: String,
    pub clipboard_context: Option<ClipboardContext>,    // this fuckass struct doesn't implement any useful trait
}

impl Debug for SceneLogin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SceneLogin")
            .field("temp_login_token", &self.temp_login_token)
            .field("url", &self.url)
            .field("clipboard_context", &Cow::from(if self.clipboard_context.is_some() {
                type_name_of_val(&self.clipboard_context)
            } else {
                "None"
            }))
            .finish()
    }
}

impl Clone for SceneLogin {
    fn clone(&self) -> Self {
        Self {
            temp_login_token: self.temp_login_token.clone(),
            url: self.url.clone(),
            clipboard_context: None,
        }
    }
}


pub fn generate_token() -> String {
    let mut buf = [0u8; 64];    // needs to be 64 because of database
    if let Err(e) = rand::rngs::OsRng.try_fill_bytes(&mut buf) {
        warn!("Could not generate cryptographically secure random bytes for token: {e}");
        rand::rng().fill_bytes(&mut buf);
    };
    let generated_token: String = base64::prelude::BASE64_URL_SAFE.encode(buf);
    generated_token
}

