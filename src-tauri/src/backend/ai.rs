use reqwest::Client;
use serde_json::{json, Value};

pub async fn generate(model: &str, prompt: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: Value = res
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(json["response"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

#[tauri::command]
pub async fn test_ai(prompt: String) -> Result<String, String> {
    generate("llama3", &prompt).await
}