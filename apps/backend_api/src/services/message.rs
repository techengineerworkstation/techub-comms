use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;

#[derive(Clone)]
pub struct MessageService {
    client: Client,
    api_key: String,
    api_secret: String,
    vonage_number: String,
}

impl MessageService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            api_key: config.vonage_api_key.clone(),
            api_secret: config.vonage_api_secret.clone(),
            vonage_number: config.vonage_number.clone(),
        }
    }

    pub async fn send_sms(&self, to: &str, from: &str, text: &str) -> Result<Value, String> {
        let resp = self.client.post("https://rest.nexmo.com/sms/json")
            .form(&[
                ("api_key", self.api_key.as_str()),
                ("api_secret", self.api_secret.as_str()),
                ("to", to), ("from", from), ("text", text),
            ])
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn send_mms(&self, to: &str, from: &str, text: &str, media_url: &[String]) -> Result<Value, String> {
        let media = media_url.first().map(|s| s.as_str()).unwrap_or("");
        let resp = self.client.post("https://rest.nexmo.com/sms/json")
            .form(&[
                ("api_key", self.api_key.as_str()),
                ("api_secret", self.api_secret.as_str()),
                ("to", to), ("from", from), ("text", text), ("media", media),
            ])
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn send_whatsapp(&self, to: &str, from: &str, text: &str) -> Result<Value, String> {
        let resp = self.client
            .post(format!("https://rest.nexmo.com/v1/messages?api_key={}&api_secret={}", self.api_key, self.api_secret))
            .json(&json!({
                "from": { "type": "whatsapp", "number": from },
                "to": { "type": "whatsapp", "number": to },
                "message_type": "text",
                "text": text
            }))
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub fn vonage_number(&self) -> &str { &self.vonage_number }
}
