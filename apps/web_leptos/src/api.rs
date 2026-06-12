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

// ─── Agora API Response Types ─────────────────────────────────────

#[derive(serde::Deserialize, Clone)]
pub struct VideoTokenResp {
    pub token: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    pub channel: String,
    pub uid: u32,
}

#[derive(serde::Deserialize, Clone)]
pub struct AgoraConfigResp {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct CallResp {
    pub channel: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    pub status: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct MsgResp {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub status: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct GenericResp {
    pub message: Option<String>,
    pub status: Option<String>,
}

// ─── Agora API Functions ───────────────────────────────────────────

pub async fn get_video_token(channel: &str, uid: u32) -> Result<VideoTokenResp, String> {
    fetch_post("/api/video/token", &serde_json::json!({
        "channel": channel,
        "uid": uid,
        "role": "publisher"
    })).await
}

pub async fn get_agora_config() -> Result<AgoraConfigResp, String> {
    fetch_get("/api/video/config").await
}

pub async fn start_recording(channel: &str, uid: u32) -> Result<GenericResp, String> {
    fetch_post("/api/video/recording/start", &serde_json::json!({
        "channel": channel,
        "uid": uid
    })).await
}

pub async fn stop_recording(resource_id: &str, sid: &str) -> Result<GenericResp, String> {
    fetch_post("/api/video/recording/stop", &serde_json::json!({
        "resourceId": resource_id,
        "sid": sid
    })).await
}

pub async fn initiate_call(to: &str, channel: Option<&str>) -> Result<CallResp, String> {
    let mut body = serde_json::json!({ "to": to });
    if let Some(ch) = channel { body["channel"] = serde_json::json!(ch); }
    fetch_post("/api/voice/call", &body).await
}

pub async fn send_call_signal(from: &str, to: &str, signal_type: &str) -> Result<GenericResp, String> {
    fetch_post("/api/voice/signal", &serde_json::json!({
        "from": from,
        "to": to,
        "signal_type": signal_type
    })).await
}

pub async fn send_text_message(from: &str, to: &str, content: &str) -> Result<MsgResp, String> {
    fetch_post("/api/message/send", &serde_json::json!({
        "from": from,
        "to": to,
        "content": content
    })).await
}

pub async fn send_image_message(from: &str, to: &str, image_url: &str) -> Result<MsgResp, String> {
    fetch_post("/api/message/send-image", &serde_json::json!({
        "from": from,
        "to": to,
        "image_url": image_url
    })).await
}

// ─── PSTN API Functions ──────────────────────────────────────────

#[derive(serde::Deserialize, Clone)]
pub struct PstnOutboundResp {
    pub success: bool,
    #[serde(rename = "callId")]
    pub call_id: Option<String>,
    pub reason: Option<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct PstnInboundResp {
    pub did: String,
    pub display: String,
    pub pin: String,
}

pub async fn pstn_outbound_call(
    to: &str,
    from: &str,
    channel: &str,
    region: &str,
) -> Result<PstnOutboundResp, String> {
    fetch_post("/api/pstn/outbound", &serde_json::json!({
        "to": to,
        "from": from,
        "channel": channel,
        "region": region,
        "prompt": "true"
    })).await
}

pub async fn pstn_inbound(channel: &str, region: &str) -> Result<PstnInboundResp, String> {
    fetch_post("/api/pstn/inbound", &serde_json::json!({
        "channel": channel,
        "region": region
    })).await
}

pub async fn pstn_end_call(call_id: &str) -> Result<GenericResp, String> {
    fetch_post("/api/pstn/end", &serde_json::json!({
        "call_id": call_id
    })).await
}
