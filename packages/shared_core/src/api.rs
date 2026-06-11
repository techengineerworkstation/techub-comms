use crate::types::*;

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    http: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn health(&self) -> Result<HealthResponse, String> {
        self.http
            .get(self.url("/health"))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    // ─── Video ────────────────────────────────────────────────────

    pub async fn get_video_session(&self, room: &str) -> Result<VideoSessionResponse, String> {
        self.http
            .get(self.url(&format!("/api/video/session/{}", room)))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn start_archive(&self, room: &str) -> Result<GenericResponse, String> {
        self.http
            .post(self.url(&format!("/api/video/session/{}/startArchive", room)))
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn stop_archive(&self, room: &str, archive_id: &str) -> Result<GenericResponse, String> {
        self.http
            .post(self.url(&format!(
                "/api/video/session/{}/{}/stopArchive",
                room, archive_id
            )))
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn list_archives(&self, room: &str) -> Result<ArchivesResponse, String> {
        self.http
            .get(self.url(&format!("/api/video/session/{}/archives", room)))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn enable_captions(&self, room: &str) -> Result<CaptionsResponse, String> {
        self.http
            .post(self.url(&format!("/api/video/session/{}/enableCaptions", room)))
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn disable_captions(&self, room: &str, captions_id: &str) -> Result<GenericResponse, String> {
        self.http
            .post(self.url(&format!(
                "/api/video/session/{}/{}/disableCaptions",
                room, captions_id
            )))
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    // ─── Voice ────────────────────────────────────────────────────

    pub async fn create_voice_call(
        &self,
        to: &str,
        from: Option<&str>,
        text: Option<&str>,
        ivr: Option<&str>,
        conference: Option<&str>,
    ) -> Result<CallResponse, String> {
        let mut body = serde_json::json!({ "to": to });
        if let Some(f) = from {
            body["from"] = serde_json::json!(f);
        }
        if let Some(t) = text {
            body["text"] = serde_json::json!(t);
        }
        if let Some(i) = ivr {
            body["ivr"] = serde_json::json!({ "prompt": i });
        }
        if let Some(c) = conference {
            body["conference"] = serde_json::json!(c);
        }
        self.http
            .post(self.url("/api/voice/call"))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn modify_voice_call(&self, uuid: &str, action: &str) -> Result<GenericResponse, String> {
        self.http
            .put(self.url(&format!("/api/voice/call/{}", uuid)))
            .json(&serde_json::json!({ "action": action }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn play_tts(&self, uuid: &str, text: &str, language: &str, voice_name: &str) -> Result<GenericResponse, String> {
        self.http
            .post(self.url(&format!("/api/voice/talk/{}", uuid)))
            .json(&serde_json::json!({
                "text": text,
                "language": language,
                "voice_name": voice_name
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn send_dtmf(&self, uuid: &str, digits: &str) -> Result<GenericResponse, String> {
        self.http
            .post(self.url(&format!("/api/voice/dtmf/{}", uuid)))
            .json(&serde_json::json!({ "digits": digits }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    // ─── Messages ─────────────────────────────────────────────────

    pub async fn send_sms(&self, to: &str, from: &str, text: &str) -> Result<MessageResponse, String> {
        self.http
            .post(self.url("/api/message/send"))
            .json(&serde_json::json!({ "to": to, "from": from, "text": text }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn send_mms(&self, to: &str, from: &str, text: &str, media_url: &[String]) -> Result<MessageResponse, String> {
        self.http
            .post(self.url("/api/message/send-mms"))
            .json(&serde_json::json!({ "to": to, "from": from, "text": text, "media_url": media_url }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn send_whatsapp(&self, to: &str, from: &str, text: &str) -> Result<MessageResponse, String> {
        self.http
            .post(self.url("/api/message/send-whatsapp"))
            .json(&serde_json::json!({ "to": to, "from": from, "text": text }))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }
}
