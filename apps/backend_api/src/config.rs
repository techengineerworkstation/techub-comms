use std::env;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub base_url: String,
    pub frontend_url: String,
    pub vonage_api_key: String,
    pub vonage_api_secret: String,
    pub vonage_application_id: String,
    pub vonage_private_key: String,
    pub vonage_number: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3039".to_string())
                .parse()
                .unwrap_or(3039),
            base_url: env::var("BASE_URL")
                .unwrap_or_else(|_| "https://api.thbtechub.sbs".to_string()),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "https://thbtechub.sbs".to_string()),
            vonage_api_key: env::var("VONAGE_API_KEY").unwrap_or_default(),
            vonage_api_secret: env::var("VONAGE_API_SECRET").unwrap_or_default(),
            vonage_application_id: env::var("VONAGE_APPLICATION_ID").unwrap_or_default(),
            vonage_private_key: resolve_private_key(),
            vonage_number: env::var("VONAGE_NUMBER").unwrap_or_default(),
        }
    }
}

fn resolve_private_key() -> String {
    if let Ok(b64) = env::var("VONAGE_PRIVATE_KEY_B64") {
        use base64::Engine;
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&b64) {
            return String::from_utf8(bytes).unwrap_or_default();
        }
    }
    if let Ok(path) = env::var("VONAGE_PRIVATE_KEY_PATH") {
        return std::fs::read_to_string(&path).unwrap_or_default();
    }
    std::fs::read_to_string("./keys/private.key").unwrap_or_default()
}
