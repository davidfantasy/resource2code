use anyhow::Result;
use async_trait::async_trait;
use rig::{
    agent::Agent,
    completion::Prompt,
    providers::openai::{self, CompletionModel},
};

use super::agent::AIAgent;

pub struct OpenAIAgent {
    agent: Agent<CompletionModel>,
}

impl OpenAIAgent {
    pub fn new(base_url: &str, api_key: &str, model: &str, preamble: &str) -> Self {
        let openai_client = openai::Client::from_url(api_key, base_url);
        let agent = openai_client.agent(model).preamble(preamble).build();
        OpenAIAgent { agent }
    }
}

#[async_trait]
impl AIAgent for OpenAIAgent {
    async fn generate_raw_response(&self, prompt: &str) -> Result<String> {
        Ok(self.agent.prompt(prompt).await?)
    }
}
