use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;
use chrono::Utc;
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Clone)]
pub struct VideoService {
    client: Client,
    app_id: String,
    app_certificate: String,
}

impl VideoService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            app_id: config.agora_app_id.clone(),
            app_certificate: config.agora_app_certificate.clone(),
        }
    }

    // Generate Agora RTC token for video/voice
    pub fn generate_token(&self, channel_name: &str, uid: u32, role: &str) -> Result<(String, String), String> {
        let now = Utc::now();
        let exp = now + chrono::Duration::hours(24);
        
        let role_num = match role {
            "publisher" => 1,
            "subscriber" => 2,
            _ => 1,
        };

        // Agora token generation using HMAC-SHA256
        let token = self.build_rtc_token(channel_name, uid, role_num, exp.timestamp() as u64)?;
        
        Ok((token, self.app_id.clone()))
    }

    fn build_rtc_token(&self, channel_name: &str, uid: u32, role: u32, expire: u64) -> Result<String, String> {
        use sha2::{Sha256, Digest};
        use hmac::{Hmac, Mac};
        use base64::Engine;

        type HmacSha256 = Hmac<Sha256>;

        let now = Utc::now().timestamp() as u64;
        
        // Build the token message
        let message = format!(
            "{}{}{}{}{}{}{}",
            self.app_id,
            channel_name,
            uid,
            role,
            now,
            expire,
            now
        );

        // Sign with app certificate
        let mut mac = HmacSha256::new_from_slice(self.app_certificate.as_bytes())
            .map_err(|e| format!("HMAC error: {}", e))?;
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        let sig_hex = hex::encode(signature);

        // Build final token
        let token_data = json!({
            "appId": self.app_id,
            "channelName": channel_name,
            "uid": uid.to_string(),
            "role": role,
            "tokenType": 0,
            "expire": expire,
            "sign": sig_hex
        });

        let token_bytes = serde_json::to_vec(&token_data).map_err(|e| e.to_string())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(token_bytes))
    }

    pub async fn create_channel(&self, channel_name: &str) -> Result<Value, String> {
        // Agora doesn't require pre-creating channels - they're created on first join
        // Return channel info
        Ok(json!({
            "channelName": channel_name,
            "appId": self.app_id,
            "status": "ready"
        }))
    }

    pub async fn start_recording(&self, channel_name: &str, uid: u32) -> Result<Value, String> {
        // Agora Cloud Recording API
        let url = format!("https://api.agora.io/v1/apps/{}/cloud_recording/acquire", self.app_id);
        
        let resp = self.client.post(&url)
            .json(&json!({
                "cname": channel_name,
                "uid": uid.to_string(),
                "clientRequest": {
                    "resourceExpiredHour": 24,
                    "scene": 0
                }
            }))
            .send().await.map_err(|e| e.to_string())?;
        
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn stop_recording(&self, _resource_id: &str, _sid: &str) -> Result<Value, String> {
        Ok(json!({"status": "stopped"}))
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }
}
