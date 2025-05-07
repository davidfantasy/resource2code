pub mod context_builder;
pub mod openai;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

pub mod agent;
mod ollama;
pub mod prompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileModifyResult {
    #[serde(rename = "filePath")]
    pub file_path: String,

    #[serde(rename = "fileContent")]
    pub file_content: String,
}

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Intent {
    ModifyFile,
    ExecuteSQL,
    Other,
}

impl FromStr for Intent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "ModifyFile" => Ok(Intent::ModifyFile),
            "ExecuteSQL" => Ok(Intent::ExecuteSQL),
            "Other" => Ok(Intent::Other),
            _ => Err(format!("Invalid forma: {}", s)),
        }
    }
}

pub fn extract_json_from_llm_response(response: &str) -> Option<String> {
    // 查找 JSON 的开始和结束位置
    let json_start = response.find('[').or_else(|| response.find('{'))?;
    let json_end = response.rfind(']').or_else(|| response.rfind('}'))?;

    // 提取 JSON 部分
    let json_str = &response[json_start..=json_end];

    // 验证 JSON 有效性
    if serde_json::from_str::<serde_json::Value>(json_str).is_ok() {
        Some(json_str.to_string())
    } else {
        // 尝试进一步清理无效字符
        let cleaned = json_str
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect::<String>();
        serde_json::from_str::<serde_json::Value>(&cleaned)
            .ok()
            .map(|_| cleaned)
    }
}
