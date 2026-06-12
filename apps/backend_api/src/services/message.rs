use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;

#[derive(Clone)]
pub struct MessageService {
    client: Client,
    app_id: String,
    app_key: String,
    org_name: String,
    app_name: String,
    rest_api: String,
    app_token: String,
}

impl MessageService {
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
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.app_token)
    }

    // Send text message via Agora Chat
    pub async fn send_text_message(&self, from: &str, to: &str, content: &str) -> Result<Value, String> {
        let url = format!("https://{}/{}/messages", self.rest_api, self.app_name);
        
        let resp = self.client.post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&json!({
                "from": from,
                "to": [to],
                "type": "txt",
                "msg": content
            }))
            .send().await.map_err(|e| e.to_string())?;
        
        let status = resp.status();
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;
        
        if status.is_success() {
            Ok(json!({
                "messageId": body.get("data").and_then(|d| d.get(to)).and_then(|v| v.as_str()).unwrap_or("sent"),
                "status": "sent"
            }))
        } else {
            Err(format!("Agora API error: {}", body))
        }
    }

    // Send image message
    pub async fn send_image_message(&self, from: &str, to: &str, image_url: &str) -> Result<Value, String> {
        let url = format!("https://{}/{}/messages", self.rest_api, self.app_name);
        
        let resp = self.client.post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&json!({
                "from": from,
                "to": [to],
                "type": "img",
                "url": image_url
            }))
            .send().await.map_err(|e| e.to_string())?;
        
        resp.json().await.map_err(|e| e.to_string())
    }

    // Create chat group
    pub async fn create_group(&self, owner: &str, group_name: &str, members: &[String]) -> Result<Value, String> {
        let url = format!("https://{}/{}/chatgroups", self.rest_api, self.app_name);
        
        let resp = self.client.post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&json!({
                "groupname": group_name,
                "desc": format!("{} group", group_name),
                "public": true,
                "maxusers": 500,
                "owner": owner,
                "members": members
            }))
            .send().await.map_err(|e| e.to_string())?;
        
        resp.json().await.map_err(|e| e.to_string())
    }

    pub fn agora_app_id(&self) -> &str {
        &self.app_id
    }
}
