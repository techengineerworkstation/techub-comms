use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;

#[derive(Clone)]
pub struct VideoService {
    client: Client,
    app_id: String,
    private_key: String,
}

impl VideoService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            app_id: config.vonage_application_id.clone(),
            private_key: config.vonage_private_key.clone(),
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

    pub async fn get_or_create_session(&self, _room: &str) -> Result<String, String> {
        let jwt = self.jwt()?;
        let resp = self.client
            .post("https://video.api.vonage.com/session/create")
            .bearer_auth(&jwt)
            .json(&json!({
                "archiveMode": "manual",
                "p2p": { "optionally": "disabled" },
                "location": {},
                "data": {}
            }))
            .send().await.map_err(|e| e.to_string())?;
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;
        body["session_id"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("No session_id: {}", body))
    }

    pub fn generate_token(&self, session_id: &str) -> Result<(String, String), String> {
        use jsonwebtoken::{encode, Header, EncodingKey};
        use chrono::Utc;

        let now = Utc::now();
        let claims = json!({
            "iss": self.app_id,
            "exp": (now + chrono::Duration::hours(1)).timestamp(),
            "iat": now.timestamp(),
            "session_id": session_id,
            "role": "moderator",
        });
        let key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())
            .map_err(|e| format!("JWT key error: {}", e))?;
        let token = encode(&Header::new(jsonwebtoken::Algorithm::RS256), &claims, &key)
            .map_err(|e| format!("JWT encode error: {}", e))?;
        Ok((token, self.app_id.clone()))
    }

    pub async fn start_archive(&self, room: &str, session_id: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client
            .post(format!("https://video.api.vonage.com/v2/project/{}/archive", self.app_id))
            .bearer_auth(&jwt)
            .json(&json!({
                "session_id": session_id,
                "name": room,
                "resolution": "1920x1200",
                "layout": { "type": "bestFit", "screenshareType": "horizontalPresentation" }
            }))
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn stop_archive(&self, archive_id: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client
            .post(format!("https://video.api.vonage.com/v2/project/{}/archive/{}/stop", self.app_id, archive_id))
            .bearer_auth(&jwt)
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn list_archives(&self, session_id: &str) -> Result<Vec<Value>, String> {
        let jwt = self.jwt()?;
        let resp = self.client
            .get(format!("https://video.api.vonage.com/v2/project/{}/archive?session_id={}", self.app_id, session_id))
            .bearer_auth(&jwt)
            .send().await.map_err(|e| e.to_string())?;
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(body["items"].as_array().cloned().unwrap_or_default())
    }

    pub async fn enable_captions(&self, session_id: &str) -> Result<Value, String> {
        let (token, _) = self.generate_token(session_id)?;
        let jwt = self.jwt()?;
        let resp = self.client
            .post(format!("https://video.api.vonage.com/v2/project/{}/captions", self.app_id))
            .bearer_auth(&jwt)
            .json(&json!({
                "session_id": session_id,
                "token": token,
                "language_code": "en-US",
                "max_duration": 1800,
                "partial_captions": "true"
            }))
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn disable_captions(&self, captions_id: &str) -> Result<Value, String> {
        let jwt = self.jwt()?;
        let resp = self.client
            .delete(format!("https://video.api.vonage.com/v2/project/{}/captions/{}", self.app_id, captions_id))
            .bearer_auth(&jwt)
            .send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }
}
