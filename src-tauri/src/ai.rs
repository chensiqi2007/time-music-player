use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub message: ChatMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub success: bool,
    pub message: String,
    pub latency_ms: u64,
}

pub struct AiClient {
    client: Client,
}

impl AiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client }
    }

    // 1. Universal OpenAI-compatible chat completion
    pub async fn chat_completion(
        &self,
        base_url: &str,
        api_key: &str,
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        let clean_base = base_url.trim().trim_end_matches('/');
        let url = if clean_base.ends_with("/chat/completions") {
            clean_base.to_string()
        } else if clean_base.ends_with("/v1") {
            format!("{}/chat/completions", clean_base)
        } else {
            format!("{}/v1/chat/completions", clean_base)
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let trimmed_key = api_key.trim();
        if !trimmed_key.is_empty() {
            let auth_val = if trimmed_key.to_lowercase().starts_with("bearer ") {
                trimmed_key.to_string()
            } else {
                format!("Bearer {}", trimmed_key)
            };
            if let Ok(val) = HeaderValue::from_str(&auth_val) {
                headers.insert(AUTHORIZATION, val);
            }
        }

        let body = ChatCompletionRequest {
            model: model.trim().to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: Some(0.3),
        };

        let res = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求 AI 接口失败: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let err_body = res.text().await.unwrap_or_default();
            return Err(format!("接口返回错误 (HTTP {}): {}", status.as_u16(), err_body));
        }

        let json: ChatCompletionResponse = res
            .json()
            .await
            .map_err(|e| format!("解析 AI 响应数据失败: {}", e))?;

        if let Some(first_choice) = json.choices.first() {
            Ok(first_choice.message.content.trim().to_string())
        } else {
            Err("AI 返回的内容为空".to_string())
        }
    }

    // 2. Test Connection
    pub async fn test_connection(
        &self,
        base_url: &str,
        api_key: &str,
        model: &str,
    ) -> Result<TestConnectionResult, String> {
        let start = Instant::now();
        let res = self
            .chat_completion(
                base_url,
                api_key,
                model,
                "You are an assistant. Reply with only one word: 'OK'.",
                "Hi, test connection.",
            )
            .await;

        let latency_ms = start.elapsed().as_millis() as u64;

        match res {
            Ok(reply) => Ok(TestConnectionResult {
                success: true,
                message: format!("连接成功 (耗时 {}ms): {}", latency_ms, reply),
                latency_ms,
            }),
            Err(e) => Ok(TestConnectionResult {
                success: false,
                message: e,
                latency_ms,
            }),
        }
    }
}

