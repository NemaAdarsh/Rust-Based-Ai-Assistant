use reqwest::Client;
use serde_json::json;









pub async fn generate_suggestion(api_key: &str, code: &str) -> String {
    let client = Client::new();
    let model_url = "https://api-inference.huggingface.co/models/bigcode/starcoder";  

    let payload = json!({ "inputs": code });

    let response = client
        .post(model_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let json_resp: serde_json::Value = resp.json().await.unwrap();
            json_resp[0]["generated_text"].as_str().unwrap_or("No response").to_string()
        }
        Err(_) => "Error reaching Hugging Face API".to_string(),
    }
}
