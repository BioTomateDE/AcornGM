mod login1;
pub use login1::MsgLogin;

#[derive(Default, Debug, Clone)]
pub struct SceneLogin {
    pub temp_login_token: String,
    pub url: String,
    pub request_listener_active: bool,
}

