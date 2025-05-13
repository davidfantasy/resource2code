use crate::{
    function::file::merge_paths,
    llm::{
        agent::{build_agent, AIAgent},
        context_builder::{CodeGenRequest, LLMContextBuilder},
        extract_json_from_llm_response,
        prompt::GENERATE_FILE_PROMPT,
        FileModifyResult,
    },
    storage::sys_config::get_config,
    task::TaskGenFile,
};

use super::{Task, TaskLog, TaskResult};
use std::{path::PathBuf, str::FromStr, sync::Arc};

use crate::task::TaskLogLevel::*;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::error;
use once_cell::sync::Lazy;
use serde_json::from_str;

static LLM_CONTEXT_BUILDER: Lazy<Arc<LLMContextBuilder>> =
    Lazy::new(|| Arc::new(LLMContextBuilder::default()));

async fn get_intent_agent() -> Result<Box<dyn AIAgent>> {
    let agent = build_agent("").await?;
    Ok(agent)
}

async fn get_code_generate_agent() -> Result<Box<dyn AIAgent>> {
    let agent = build_agent(GENERATE_FILE_PROMPT).await?;
    Ok(agent)
}

pub struct CodeGenTask {
    req: CodeGenRequest,
    is_cancelled: Arc<std::sync::atomic::AtomicBool>,
}

impl CodeGenTask {
    pub fn new(req: CodeGenRequest) -> Self {
        Self {
            req,
            is_cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    fn check_cancelled(&self) -> Result<()> {
        if self.is_cancelled.load(std::sync::atomic::Ordering::Relaxed) {
            Err(anyhow!("任务已被取消"))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Intent {
    CodeGen,
    ExecuteSQL,
    Other,
}

impl FromStr for Intent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "CodeGen" => Ok(Intent::CodeGen),
            "ExecuteSQL" => Ok(Intent::ExecuteSQL),
            "Other" => Ok(Intent::Other),
            _ => Err(format!("Invalid forma: {}", s)),
        }
    }
}

#[async_trait]
impl Task for CodeGenTask {
    async fn start(&mut self, sender: tokio::sync::mpsc::Sender<TaskLog>) -> Result<TaskResult> {
        self.check_cancelled()?;
        self.send_log(&sender, "开始执行代码生成任务").await?;

        let intent = self.analyze_intent(&sender).await?;
        if intent != Intent::CodeGen {
            return Err(anyhow!("当前不支持该类型的问题处理"));
        }

        let context = self.build_context(&sender).await?;
        let agent = get_code_generate_agent().await?;
        const MAX_RETRIES: usize = 3;
        let mut retry_count = 0;
        loop {
            let res = self.query_llm(&sender, &agent, &context).await?;
            match self.process_llm_response(&res).await {
                Ok(result) => break Ok(result),
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= MAX_RETRIES {
                        break Err(e);
                    }
                    self.send_log(
                        &sender,
                        &format!(
                            "LLM回复格式错误，正在重试 ({}/{})",
                            retry_count, MAX_RETRIES
                        ),
                    )
                    .await?;
                }
            }
        }
    }

    async fn cancel(&mut self) -> anyhow::Result<()> {
        self.is_cancelled
            .store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl CodeGenTask {
    async fn send_log(
        &self,
        sender: &tokio::sync::mpsc::Sender<TaskLog>,
        message: &str,
    ) -> Result<()> {
        self.check_cancelled()?;
        sender.send(TaskLog::new(message, Info)).await?;
        Ok(())
    }

    async fn analyze_intent(&self, sender: &tokio::sync::mpsc::Sender<TaskLog>) -> Result<Intent> {
        self.send_log(sender, "正在分析用户意图").await?;
        let intent = analyze_intent(&self.req.question).await?;
        Ok(intent)
    }

    async fn build_context(&self, sender: &tokio::sync::mpsc::Sender<TaskLog>) -> Result<String> {
        self.send_log(sender, "正在构建与问题相关联的上下文")
            .await?;
        let context = LLM_CONTEXT_BUILDER.build(&self.req).await?;
        self.send_log(sender, "上下文已构建完成").await?;
        Ok(context)
    }

    async fn query_llm(
        &self,
        sender: &tokio::sync::mpsc::Sender<TaskLog>,
        agent: &Box<dyn AIAgent>,
        context: &str,
    ) -> Result<String> {
        self.send_log(sender, "开始提交问题到LLM").await?;
        let res = agent.generate_response(context).await?;
        self.send_log(sender, "LLM已完成回答").await?;
        Ok(res)
    }

    async fn process_llm_response(&self, response: &str) -> Result<TaskResult> {
        let json_str = extract_json_from_llm_response(response)
            .ok_or_else(|| anyhow!("LLM响应格式错误：{}", response))?;
        let file_modify_results: Vec<FileModifyResult> = from_str(&json_str).map_err(|e| {
            error!("JSON解析详细错误: {:?}\n原始JSON字符串: {}", e, json_str);
            anyhow!("LLM返回数据格式错误: {}, 原始内容: {}", e, json_str)
        })?;
        let root_dir = get_config("root_source_path".to_string())
            .await?
            .unwrap_or("".to_string());
        let files = file_modify_results
            .into_iter()
            .map(|result| {
                let file_path = PathBuf::from(result.file_path.clone());
                TaskGenFile {
                    name: file_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    path: Some(merge_paths(&root_dir, &result.file_path)),
                    content: result.file_content,
                }
            })
            .collect();
        Ok(TaskResult::CodeGen { files })
    }
}

async fn analyze_intent(user_question: &str) -> Result<Intent> {
    let prompt = format!(
        "Analyze the user's question and determine the intent. 
        Question: \"{}\"
        
        Possible intent classifications:
        1. If the question involves generating or modifying code, respond with exactly \"CodeGen\"
        2. If the question involves executing SQL queries, respond with exactly \"ExecuteSQL\"
        3. For all other cases, respond with exactly \"Other\"
        
        Important:
        - Only respond with one of the exact enum values: ModifyFile, ExecuteSQL or Other
        - Do not include any additional text or explanation",
        user_question
    );

    let agent = get_intent_agent().await?;
    let llm_response = agent.generate_response(&prompt).await?;
    let llm_response = llm_response.trim().trim_matches('"').to_string();
    Intent::from_str(&llm_response)
        .map_err(|_| anyhow!("LLM returned invalid intent format: \"{}\"", llm_response))
}
