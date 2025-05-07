use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::storage::sys_config::get_config;

use super::{ollama::OllamaAgent, openai::OpenAIAgent};

#[async_trait]
pub trait AIAgent: Send + Sync {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let raw_response = self.generate_raw_response(prompt).await?;
        Ok(remove_think_tags(&raw_response))
    }

    async fn generate_raw_response(&self, prompt: &str) -> Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMProvider {
    #[serde(rename = "name")]
    pub name: LLMProviderType,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "maxTokens")]
    pub max_tokens: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProviderType {
    OpenAI,
    Ollama,
}

pub async fn build_agent(preamble: &str) -> Result<Box<dyn AIAgent>> {
    let llm_provider_conf = get_config("current_llm_provider".to_string()).await?;
    if llm_provider_conf.is_none() {
        return Err(anyhow::anyhow!("当前未配置LLM供应商"));
    }
    let llm_provider: LLMProvider = serde_json::from_str(&llm_provider_conf.unwrap())?;
    match llm_provider.name {
        LLMProviderType::OpenAI => {
            let agent = OpenAIAgent::new(
                &llm_provider.base_url,
                &llm_provider.api_key,
                &llm_provider.model,
                preamble,
            );
            Ok(Box::new(agent))
        }
        LLMProviderType::Ollama => {
            let agent = OllamaAgent::new(&llm_provider.base_url, &llm_provider.model, preamble);
            Ok(Box::new(agent))
        }
    }
}

/// 去除LLM回复中的<think></think>标签及其内容
pub fn remove_think_tags(response: &str) -> String {
    let mut result = String::new();
    let mut in_think_tag = false;

    let mut chars = response.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            // 检查是否是<think>开始标签
            if chars.clone().take(6).collect::<String>().to_lowercase() == "think>" {
                in_think_tag = true;
                // 跳过"think>"这6个字符
                for _ in 0..6 {
                    chars.next();
                }
                continue;
            }
            // 检查是否是</think>结束标签
            else if chars.clone().take(7).collect::<String>().to_lowercase() == "/think>" {
                in_think_tag = false;
                // 跳过"/think>"这7个字符
                for _ in 0..7 {
                    chars.next();
                }
                continue;
            }
        }

        if !in_think_tag {
            result.push(c);
        }
    }

    result
}
