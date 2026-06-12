use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;
use base64::Engine;

#[derive(Clone)]
pub struct VoiceService {
    client: Client,
    app_id: String,
    app_key: String,
    org_name: String,
    app_name: String,
    rest_api: String,
    app_token: String,
    base_url: String,
}

impl VoiceService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            app_id: config.agora_app_id.clone(),
            app_key: config.agora_app_key.clone(),
            org_name: config.agora_org_name.clone(),
            app_name: config.agora_app_name.clone(),
            rest_api: config.agora_rest_api.clone(),
            app_token: config.agora_chat_app_token.clone(),
            base_url: config.base_url.clone(),
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.app_token)
    }

    // Create Agora Chat user for voice messaging
    pub async fn create_user(&self, username: &str, password: &str) -> Result<Value, String> {
        let url = format!("https://{}/{}/users", self.rest_api, self.app_name);
        
        let resp = self.client.post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&json!({
                "user": username,
                "password": password
            }))
            .send().await.map_err(|e| e.to_string())?;
        
        resp.json().await.map_err(|e| e.to_string())
    }

    // Send signaling message for voice call
    pub async fn send_call_signal(&self, from: &str, to: &str, signal_type: &str) -> Result<Value, String> {
        let url = format!("https://{}/{}/messages", self.rest_api, self.app_name);
        
        let resp = self.client.post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&json!({
                "from": from,
                "to": [to],
                "type": "txt",
                "msg": format!("{}:call", signal_type)
            }))
            .send().await.map_err(|e| e.to_string())?;
        
        resp.json().await.map_err(|e| e.to_string())
    }

    // Create voice call room
    pub async fn create_call_room(&self, room_name: &str) -> Result<Value, String> {
        // Agora uses channels - no pre-creation needed
        Ok(json!({
            "channelName": room_name,
            "appId": self.app_id,
            "status": "ready"
        }))
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn agora_app_id(&self) -> &str {
        &self.app_id
    }
}
