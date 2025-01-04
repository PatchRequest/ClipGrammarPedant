use clipboard::{ClipboardContext, ClipboardProvider};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct ChatRequestMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatRequestMessage>,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatResponseChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatResponseChoice>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut clipboard: ClipboardContext = ClipboardProvider::new()?;

    // Get text from clipboard
    let text = clipboard.get_contents()?;
    println!("Original text: {}", text);

    // Read API key from environment variable
    let api_key = env::var("OPENAI_API_KEY")?;

    // Prepare the request
    let api_url = "https://api.openai.com/v1/chat/completions";

    let request = ChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![ChatRequestMessage {
            role: "user".to_string(),
            content: format!("Please carefully review the following text for any grammar, spelling, or punctuation errors and correct them. Ensure the meaning and tone remain consistent with the original. Provide only the revised text as your response, without any additional comments or explanations:
            
            {}", text),
        }],
        temperature: 0.7,
    };

    let client = Client::new();

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()?;

    if response.status().is_success() {
        let chat_response: ChatResponse = response.json()?;
        let corrected_text = &chat_response.choices[0].message.content;
        println!("Corrected text: {}", corrected_text);

        // Copy corrected text back to clipboard
        clipboard.set_contents(corrected_text.clone())?;
        println!("Corrected text copied to clipboard.");
    } else {
        eprintln!("Failed to get a response: {}", response.text()?);
    }

    Ok(())
}
