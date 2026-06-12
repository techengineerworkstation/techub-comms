use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    // Agora credentials
    pub agora_app_id: String,
    pub agora_app_certificate: String,
    pub agora_app_key: String,
    pub agora_org_name: String,
    pub agora_app_name: String,
    pub agora_rest_api: String,
    pub agora_chat_app_token: String,
    pub agora_chat_user_token: String,
    // Agora SIP/PSTN
    pub agora_sip_auth_token: String,
    // Server config
    pub server_port: u16,
    pub base_url: String,
    pub frontend_url: String,
    pub api_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            agora_app_id: env::var("AGORA_APP_ID")
                .unwrap_or_else(|_| "41200037042".to_string()),
            agora_app_certificate: env::var("AGORA_APP_CERTIFICATE")
                .unwrap_or_default(),
            agora_app_key: env::var("AGORA_APP_KEY")
                .unwrap_or_else(|_| "41200037042#200051503".to_string()),
            agora_org_name: env::var("AGORA_ORG_NAME")
                .unwrap_or_else(|_| "41200037042".to_string()),
            agora_app_name: env::var("AGORA_APP_NAME")
                .unwrap_or_else(|_| "200051503".to_string()),
            agora_rest_api: env::var("AGORA_REST_API")
                .unwrap_or_else(|_| "a41.chat.agora.io".to_string()),
            agora_chat_app_token: env::var("AGORA_CHAT_APP_TOKEN")
                .unwrap_or_else(|_| "007eJxTYHi6yGWlUAv/30fTS5Zv/7D+ySPn7btm1zG/Sl26wW3umx9mCgyJhmmGqUkWxoaJKeYmqYnJSammaQaJxqmG5kap5kmJad+8dLIaAhkZ5MPEGBgZWIGYiQHEZ2AAALyxIVU=".to_string()),
            agora_chat_user_token: env::var("AGORA_CHAT_USER_TOKEN")
                .unwrap_or_else(|_| "007eJxTYDC+ojchac21HLMzenZCAhU9n+dpyKrIepZG6pvNk+lzvaDAkGiYZpiaZGFsmJhibpKamJyUappmkGicamhulGqelJj23ksnqyGQkWFxqC0LIwMrAyMQgvgqDBYpKSYmqSkGumZmFpa6hkBzdBOTLE10U5LSjMxNjc1Mk5JMAGfVJPw=".to_string()),
            agora_sip_auth_token: env::var("AGORA_SIP_AUTH_TOKEN")
                .unwrap_or_default(),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3039".to_string())
                .parse()
                .unwrap_or(3039),
            base_url: env::var("BASE_URL")
                .unwrap_or_else(|_| "https://api.thbtechub.sbs".to_string()),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "https://thbtechub.sbs".to_string()),
            api_url: env::var("API_URL")
                .unwrap_or_else(|_| "https://api.thbtechub.sbs".to_string()),
        }
    }
}
