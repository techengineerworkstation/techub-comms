use reqwest::Client;
use serde_json::{json, Value};
use shared_core::AppConfig;

#[derive(Clone)]
pub struct PstnService {
    client: Client,
    app_id: String,
    sip_auth_token: String,
    base_url: String,
}

impl PstnService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            app_id: config.agora_app_id.clone(),
            sip_auth_token: config.agora_sip_auth_token.clone(),
            base_url: "https://sipcm.agora.io/v1/api/pstn".to_string(),
        }
    }

    // Outbound PSTN call - dials a phone number and connects to Agora channel
    pub async fn outbound_call(
        &self,
        to: &str,
        from: &str,
        channel: &str,
        uid: &str,
        region: &str,
        prompt: &str,
    ) -> Result<Value, String> {
        let resp = self.client.post(&self.base_url)
            .header("Authorization", format!("Basic {}", self.sip_auth_token))
            .header("Content-Type", "application/json")
            .json(&json!({
                "action": "outbound",
                "appid": self.app_id,
                "region": region,
                "uid": uid,
                "channel": channel,
                "from": from,
                "to": to,
                "prompt": prompt
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = resp.status();
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(format!("PSTN API error ({}): {}", status, body))
        }
    }

    // Inbound PSTN - get a phone number and PIN for users to dial in
    pub async fn inbound_pstn(
        &self,
        channel: &str,
        uid: &str,
        region: &str,
    ) -> Result<Value, String> {
        let resp = self.client.post(&self.base_url)
            .header("Authorization", format!("Basic {}", self.sip_auth_token))
            .header("Content-Type", "application/json")
            .json(&json!({
                "action": "inbound",
                "appid": self.app_id,
                "uid": uid,
                "channel": channel,
                "region": region
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = resp.status();
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(format!("PSTN API error ({}): {}", status, body))
        }
    }

    // End an active PSTN call
    pub async fn end_call(&self, call_id: &str) -> Result<Value, String> {
        let resp = self.client.post(&self.base_url)
            .header("Authorization", format!("Basic {}", self.sip_auth_token))
            .header("Content-Type", "application/json")
            .json(&json!({
                "action": "endcall",
                "appid": self.app_id,
                "callid": call_id
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = resp.status();
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(format!("PSTN API error ({}): {}", status, body))
        }
    }

    // Cancel a pending PSTN call
    pub async fn cancel_call(&self, call_id: &str) -> Result<Value, String> {
        let resp = self.client.post(&self.base_url)
            .header("Authorization", format!("Basic {}", self.sip_auth_token))
            .header("Content-Type", "application/json")
            .json(&json!({
                "action": "cancelcall",
                "appid": self.app_id,
                "callid": call_id
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = resp.status();
        let body: Value = resp.json().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(format!("PSTN API error ({}): {}", status, body))
        }
    }

    // Get SIP entrypoint for a region
    pub fn sip_entrypoint(&self, region: &str, with_video: bool, transport: &str) -> String {
        let host = match region {
            "EU" => "sip.eu.lb.01.agora.io",
            "NA" | "US" => "sip.usa.lb.01.agora.io",
            "AS" | "ASIA" => "sip.as.lb.01.agora.io",
            _ => "sip.usa.lb.01.agora.io",
        };

        let port = match (with_video, transport) {
            (false, "tls") => 5081,
            (false, _) => 5080,
            (true, "tls") => 5091,
            (true, _) => 5090,
        };

        if transport == "tls" {
            format!("{}:{};transport=tls", host, port)
        } else {
            format!("{}:{}", host, port)
        }
    }
}
