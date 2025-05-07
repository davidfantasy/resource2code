use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::{
    datasource::get_table_schema,
    storage::{code_sample::get_sample_by_id, datasource::get_ds_by_id},
};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct CodeGenRequest {
    pub question: String,
    #[serde(rename = "sampleIds")]
    pub sample_ids: Vec<String>,
    pub resources: Vec<ResourceMeta>,
    #[serde(rename = "autoDetectDir")]
    pub auto_detect_dir: bool,
    #[serde(rename = "currentSrcDir")]
    pub current_src_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct ResourceMeta {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub name: String,
    pub data: String,
}

struct ResourceProcessor {}

pub struct LLMContextBuilder {
    processor: ResourceProcessor,
}

impl Default for LLMContextBuilder {
    fn default() -> Self {
        Self {
            processor: ResourceProcessor {},
        }
    }
}

impl ResourceProcessor {
    async fn process(&self, resource: &ResourceMeta) -> Result<String> {
        match resource.resource_type.as_str() {
            "table" => self.process_table(resource).await,
            "file" => self.process_file(resource).await,
            _ => Err(anyhow!("Unsupported resource type")),
        }
    }

    async fn process_table(&self, resource: &ResourceMeta) -> Result<String> {
        let ds_id = resource.data.clone();
        let ds = get_ds_by_id(ds_id).await?;
        let schema = get_table_schema(ds, resource.name.clone()).await?;
        Ok(format!("##引用数据表schema: {}\n", schema))
    }

    async fn process_file(&self, resource: &ResourceMeta) -> Result<String> {
        let content = fs::read_to_string(&resource.name)?;
        Ok(format!(
            "##引用文件： {}\n```\n{}\n```",
            resource.name, content
        ))
    }
}

impl LLMContextBuilder {
    pub async fn build(&self, request: &CodeGenRequest) -> Result<String> {
        let mut context = String::new();
        // 用户问题
        context.push_str(&format!("#用户问题：\"{}\"\n\n", request.question));
        // 代码示例
        if !request.sample_ids.is_empty() {
            for simple_id in &request.sample_ids {
                let sample = get_sample_by_id(simple_id).await?;
                context.push_str(&format!(
                    "##引用代码示例： {}\n```\n{}\n```",
                    sample.name, sample.content
                ));
            }
        }
        //资源内容
        for resource in &request.resources {
            let content = self.processor.process(resource).await?;
            if resource.resource_type == "table" {
                context.push_str(&format!(
                    "##引用数据表schema：{}\n```\n{}\n```",
                    resource.name, content
                ));
            }
            if resource.resource_type == "file" {
                context.push_str(&format!(
                    "##引用代码文件内容：{}\n```\n{}\n```",
                    resource.data, content
                ));
            }
        }
        if request.auto_detect_dir {
            let dir_structure =
                generate_directory_structure(PathBuf::from(&request.current_src_dir));
            if !dir_structure.is_empty() {
                context.push_str("#当前源码目录结构：");
                context.push_str(&dir_structure);
            }
        }
        Ok(context)
    }
}

//基于指定的根目录，生成根目录的整个目录结构树状图文本，用于附加到LLM的上下文中。
fn generate_directory_structure(root_dir: PathBuf) -> String {
    let mut dir_structure = String::new();
    dir_structure.push_str("```\n");
    for entry in WalkDir::new(&root_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() && path != root_dir {
            // 排除根目录本身
            let relative_path = path.strip_prefix(&root_dir).unwrap();
            dir_structure.push_str(&format!(
                "/{}\n",
                relative_path.to_string_lossy().replace('\\', "/") // 统一使用/作为路径分隔符
            ));
        }
    }
    dir_structure.push_str("```\n");
    dir_structure
}
