use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSession {
    pub session_id: String,
    pub token: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captions_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archive {
    pub id: String,
    pub name: String,
    pub session_id: String,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCall {
    pub uuid: String,
    #[serde(default)]
    pub conversation_uuid: Option<String>,
    pub status: String,
    pub direction: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub to: String,
    pub from: String,
    pub channel: String,
    pub text: String,
    #[serde(default)]
    pub media_url: Option<String>,
    pub status: String,
    pub timestamp: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    #[serde(default)]
    pub connection_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    pub has_video: bool,
    pub has_audio: bool,
    pub is_screen_sharing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NCCOAction {
    pub action: String,
    #[serde(flatten)]
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSessionResponse {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub token: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivesResponse {
    #[serde(default)]
    pub archives: Vec<Archive>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResponse {
    pub uuid: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionsResponse {
    #[serde(rename = "captionsId")]
    pub captions_id: String,
    pub status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericResponse {
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
