# AI Code Assistant (Rust)

AI Code Assistant is a command-line tool built with Rust that integrates with the Hugging Face API to provide AI-generated code suggestions based on user input. The tool allows developers to type code snippets and receive AI-generated recommendations in real time.

## Features
- Interactive CLI for AI-assisted coding
- Supports Rust code suggestions using Hugging Face models
- Asynchronous API calls with `tokio`

## Installation
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/rust-ai-code-assistant.git
   cd rust-ai-code-assistant
   ```
2. Install dependencies:
   ```sh
   cargo build
   ```
3. Run the project:
   ```sh
   cargo run
   ```

## Usage
1. Start the assistant:
   ```sh
   cargo run
   ```
2. Enter your Hugging Face API key when prompted.
3. Type a Rust code snippet and receive AI-generated suggestions.
4. Type `exit` to quit.

### Example Session
```
Enter your Hugging Face API key: xxxxxxxx
Welcome to AI Code Assistant! Type code and get AI suggestions.

>>> struct User { name: String, age: u32 }
AI Suggestion:
impl User {
    fn new(name: &str, age: u32) -> Self {
        Self { name: name.to_string(), age }
    }
}

>>> exit
```

## Development
### Folder Structure
```
├── src
│   ├── main.rs      # CLI logic
│   ├── ai.rs        # API request handling
│   ├── lib.rs       # Additional helper functions
│
├── Cargo.toml       # Dependencies and metadata
```
### Modifying the AI Response Format
To improve or change AI responses, modify `ai.rs`:
```rust
pub async fn generate_suggestion(api_key: &str, input: &str) -> String {
    let client = reqwest::Client::new();
    let request_body = serde_json::json!({
        "inputs": format!("Complete the following Rust code:

{}", input),
        "parameters": {
            "max_new_tokens": 200,
            "temperature": 0.7
        }
    });
    
    match client
        .post("https://api-inference.huggingface.co/models/YOUR_MODEL")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => response.text().await.unwrap_or("Error".to_string()),
        Err(_) => "API request failed".to_string(),
    }
}
```

