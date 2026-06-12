use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;

fn api_base() -> String {
    let win = web_sys::window().expect("no window");
    let loc = win.location();
    let origin = loc.origin().unwrap_or_default();
    if origin.contains("localhost") || origin.contains("127.0.0.1") {
        "http://localhost:3039".to_string()
    } else {
        origin
    }
}

async fn fetch_get<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = format!("{}{}", api_base(), path);
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let req = Request::new_with_str_and_init(&url, &opts).map_err(|e| format!("{:?}", e))?;
    req.headers().set("Accept", "application/json").ok();
    let win = web_sys::window().expect("no window");
    let resp_val = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|e| format!("{:?}", e))?;
    let resp: Response = resp_val.dyn_into().map_err(|_| "bad response".to_string())?;
    if !resp.ok() { return Err(format!("HTTP {}", resp.status())); }
    let json_val = JsFuture::from(resp.json().map_err(|e| format!("{:?}", e))?).await.map_err(|e| format!("{:?}", e))?;
    serde_wasm_bindgen::from_value(json_val).map_err(|e| format!("{:?}", e))
}

async fn fetch_post<T: serde::de::DeserializeOwned, B: serde::Serialize>(path: &str, body: &B) -> Result<T, String> {
    let url = format!("{}{}", api_base(), path);
    let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body_str));
    let req = Request::new_with_str_and_init(&url, &opts).map_err(|e| format!("{:?}", e))?;
    req.headers().set("Content-Type", "application/json").ok();
    req.headers().set("Accept", "application/json").ok();
    let win = web_sys::window().expect("no window");
    let resp_val = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|e| format!("{:?}", e))?;
    let resp: Response = resp_val.dyn_into().map_err(|_| "bad response".to_string())?;
    if !resp.ok() { return Err(format!("HTTP {}", resp.status())); }
    let json_val = JsFuture::from(resp.json().map_err(|e| format!("{:?}", e))?).await.map_err(|e| format!("{:?}", e))?;
    serde_wasm_bindgen::from_value(json_val).map_err(|e| format!("{:?}", e))
}

async fn fetch_put<T: serde::de::DeserializeOwned, B: serde::Serialize>(path: &str, body: &B) -> Result<T, String> {
    let url = format!("{}{}", api_base(), path);
    let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
    let opts = RequestInit::new();
    opts.set_method("PUT");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body_str));
    let req = Request::new_with_str_and_init(&url, &opts).map_err(|e| format!("{:?}", e))?;
    req.headers().set("Content-Type", "application/json").ok();
    let win = web_sys::window().expect("no window");
    let resp_val = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|e| format!("{:?}", e))?;
    let resp: Response = resp_val.dyn_into().map_err(|_| "bad response".to_string())?;
    if !resp.ok() { return Err(format!("HTTP {}", resp.status())); }
    let json_val = JsFuture::from(resp.json().map_err(|e| format!("{:?}", e))?).await.map_err(|e| format!("{:?}", e))?;
    serde_wasm_bindgen::from_value(json_val).map_err(|e| format!("{:?}", e))
}

// ─── API Response Types ─────────────────────────────────────────────

#[derive(serde::Deserialize, Clone)]
pub struct VideoSessionResp {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub token: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(serde::Deserialize)]
pub struct ArchivesResp {
    pub archives: Vec<ArchiveResp>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ArchiveResp {
    pub id: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub duration: Option<f64>,
    pub url: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CallResp {
    pub uuid: String,
    pub status: String,
}

#[derive(serde::Deserialize)]
pub struct MsgResp {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub status: String,
}

#[derive(serde::Deserialize)]
pub struct CaptionsResp {
    #[serde(rename = "captionsId")]
    pub captions_id: String,
}

#[derive(serde::Deserialize)]
pub struct GenericResp {
    pub message: Option<String>,
}

// ─── API Functions ───────────────────────────────────────────────────

pub async fn get_session(room: &str) -> Result<VideoSessionResp, String> {
    fetch_get(&format!("/api/video/session/{}", room)).await
}

pub async fn start_archive(room: &str) -> Result<GenericResp, String> {
    fetch_post(&format!("/api/video/session/{}/startArchive", room), &serde_json::json!({})).await
}

pub async fn stop_archive(room: &str, aid: &str) -> Result<GenericResp, String> {
    fetch_post(&format!("/api/video/session/{}/{}/stopArchive", room, aid), &serde_json::json!({})).await
}

pub async fn list_archives(room: &str) -> Result<ArchivesResp, String> {
    fetch_get(&format!("/api/video/session/{}/archives", room)).await
}

pub async fn enable_captions(room: &str) -> Result<CaptionsResp, String> {
    fetch_post(&format!("/api/video/session/{}/enableCaptions", room), &serde_json::json!({})).await
}

pub async fn disable_captions(room: &str, cid: &str) -> Result<GenericResp, String> {
    fetch_post(&format!("/api/video/session/{}/{}/disableCaptions", room, cid), &serde_json::json!({})).await
}

pub async fn create_call(to: &str, text: Option<&str>) -> Result<CallResp, String> {
    let mut body = serde_json::json!({ "to": to });
    if let Some(t) = text { body["text"] = serde_json::json!(t); }
    fetch_post("/api/voice/call", &body).await
}

pub async fn modify_call(uuid: &str, action: &str) -> Result<GenericResp, String> {
    fetch_put(&format!("/api/voice/call/{}", uuid), &serde_json::json!({ "action": action })).await
}

pub async fn play_tts(uuid: &str, text: &str) -> Result<GenericResp, String> {
    fetch_post(&format!("/api/voice/talk/{}", uuid), &serde_json::json!({
        "text": text,
        "language": "en-US",
        "voice_name": "Amy"
    })).await
}

pub async fn send_dtmf_api(uuid: &str, digits: &str) -> Result<GenericResp, String> {
    fetch_post(&format!("/api/voice/dtmf/{}", uuid), &serde_json::json!({ "digits": digits })).await
}

pub async fn send_sms_api(to: &str, text: &str) -> Result<MsgResp, String> {
    fetch_post("/api/message/send", &serde_json::json!({ "to": to, "text": text })).await
}

pub async fn send_whatsapp_api(to: &str, text: &str) -> Result<MsgResp, String> {
    fetch_post("/api/message/send-whatsapp", &serde_json::json!({ "to": to, "text": text })).await
}
