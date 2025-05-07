use anyhow::Result;
use async_trait::async_trait;
use rig::{
    agent::Agent,
    completion::Prompt,
    providers::ollama::{self, CompletionModel},
};

use super::agent::AIAgent;

pub struct OllamaAgent {
    agent: Agent<CompletionModel>,
}

impl OllamaAgent {
    pub fn new(base_url: &str, model: &str, preamble: &str) -> Self {
        let ollama_client = ollama::Client::from_url(base_url);
        let agent = ollama_client.agent(model).preamble(preamble).build();
        OllamaAgent { agent }
    }
}

#[async_trait]
impl AIAgent for OllamaAgent {
    async fn generate_raw_response(&self, prompt: &str) -> Result<String> {
        Ok(self.agent.prompt(prompt).await?)
    }
}
