use std::vec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

pub struct Client {
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
        }
    }

    pub async fn chat(&self, model: &str, user_messages: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();

        let request_body = RequestBody {
            model: model.to_string(),
            messages: vec![
                Message::new("developer", "You are a terminal command tool, to directly output commands to achieve the user's requirement. You can only output the command, but you can add ONE very short annotation after commands only if there are some tips. If you don't think there is any way to achieve or the requirements from user are not correlative to commands, output 'NULL'. DO NOT USE '\\n'. Your commands are defaultly based on Bash Shell. It's no need for you to output commands in code block."),
                Message::new("user", user_messages)
            ]
        };

        let response = client.post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let body = response.text().await?;
        let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();

        let response_content = json_body["choices"][0]["message"]["content"].as_str().unwrap();
        Ok(response_content.to_string())

    }
}

