use serde_json::{json, Value};

pub fn talk(text: &str, barge_in: bool) -> Value {
    json!({
        "action": "talk", "text": text,
        "language": "en-US", "voiceName": "Amy",
        "loop": 1, "bargeIn": barge_in,
    })
}

pub fn input(event_url: Vec<String>, max_digits: u32) -> Value {
    json!({
        "action": "input",
        "type": ["dtmf"],
        "dtmf": { "maxDigits": max_digits, "timeOut": 5 },
        "eventUrl": event_url,
        "eventMethod": "POST",
    })
}

pub fn connect(number: &str, base_url: &str, vonage_number: &str) -> Value {
    json!({
        "action": "connect",
        "from": vonage_number,
        "endpoint": [{ "type": "phone", "number": number }],
        "timeout": 30,
        "eventUrl": [format!("{}/webhooks/event", base_url)],
        "eventMethod": "POST",
    })
}

pub fn record(base_url: &str) -> Value {
    json!({
        "action": "record", "format": "mp3",
        "timeOut": 30, "endOnSilence": 3, "endOnKey": "#",
        "eventUrl": [format!("{}/webhooks/recording", base_url)],
        "eventMethod": "POST",
    })
}

pub fn conversation(name: &str) -> Value {
    json!({
        "action": "conversation", "name": name,
        "startOnEnter": true, "endOnExit": false, "record": false,
    })
}

pub fn ivr_menu(prompt: &str, event_url: Vec<String>) -> Vec<Value> {
    vec![talk(prompt, false), input(event_url, 1)]
}
