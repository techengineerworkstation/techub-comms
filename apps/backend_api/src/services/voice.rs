use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;

#[derive(Clone)]
pub struct VoiceService {
    client: Client,
    app_id: String,
    private_key: String,
    base_url: String,
    vonage_number: String,
}

impl VoiceService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            app_id: config.vonage_application_id.clone(),
            private_key: config.vonage_private_key.clone(),
            base_url: config.base_url.clone(),
            vonage_number: config.vonage_number.clone(),
        }
    }

    fn jwt(&self) -> Result<String, String> {
        use jsonwebtoken::{encode, Header, EncodingKey};
        use chrono::Utc;
        let now = Utc::now();
        let claims = json!({
            "iss": self.app_id,
            "exp": (now + chrono::Duration::hours(1)).timestamp(),
            "iat": now.timestamp(),
        });
        let key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())
            .map_err(|e| format!("JWT key error: {}", e))?;
        encode(&Header::new(jsonwebtoken::Algorithm::RS256), &claims, &key)
            .map_err(|e| format!("JWT encode error: {}", e))
    }

    pub async fn create_outbound_call(
        &self, to: &str, from: &str, ncco: Vec<Value>,
        answer_url: Option<String>, event_url: Option<String>,
    ) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let mut payload = json!({
            "to": [{ "type": "phone", "number": to }],
            "from": { "type": "phone", "number": from },
        });
        if !ncco.is_empty() { payload["ncco"] = json!(ncco); }
        if let Some(u) = answer_url { payload["answer_url"] = json!([u]); payload["answer_method"] = json!("POST"); }
        if let Some(u) = event_url { payload["event_url"] = json!([u]); payload["event_method"] = json!("POST"); }

        let resp = self.client.post("https://rest.nexmo.com/v1/calls")
            .bearer_auth(&jwt).json(&payload)
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn modify_call(&self, uuid: &str, action: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client.put(format!("https://rest.nexmo.com/v1/calls/{}", uuid))
            .bearer_auth(&jwt).json(&json!({ "action": action }))
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn play_tts(&self, uuid: &str, text: &str, language: &str, voice_name: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client.put(format!("https://rest.nexmo.com/v1/calls/{}/talk", uuid))
            .bearer_auth(&jwt).json(&json!({ "text": text, "language": language, "voice_name": voice_name, "level": 0, "loop": 1 }))
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn stop_tts(&self, uuid: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client.delete(format!("https://rest.nexmo.com/v1/calls/{}/talk", uuid))
            .bearer_auth(&jwt)
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn send_dtmf(&self, uuid: &str, digits: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client.put(format!("https://rest.nexmo.com/v1/calls/{}/dtmf", uuid))
            .bearer_auth(&jwt).json(&json!({ "digits": digits }))
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub fn base_url(&self) -> &str { &self.base_url }
    pub fn vonage_number(&self) -> &str { &self.vonage_number }
}
