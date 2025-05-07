use db::get_tables;
use function::file::{file_existed, save_file};
use llm::context_builder::CodeGenRequest;
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self};
use std::path::{Path, PathBuf};
use storage::code_sample::*;
use storage::datasource::*;
use storage::init_db;
use storage::sys_config::*;
use task::code_gen_task::CodeGenTask;
use task::{periodic_cleanup_inactive_tasks, TaskLog, TaskResult};
pub mod datasource;
pub mod db;
pub mod function;
pub mod llm;
pub mod storage;
mod task;

use anyhow::Error;

trait ToTauriResult<T> {
    fn to_tauri_result(self) -> Result<T, String>;
}

impl<T> ToTauriResult<T> for Result<T, Error> {
    fn to_tauri_result(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    init_db().await.unwrap();
    init_log();
    //定时清理过期的用户任务
    periodic_cleanup_inactive_tasks(120);
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_file_system,
            get_tables,
            create_ds,
            delete_ds,
            get_all_ds,
            get_ds_by_id,
            update_ds,
            create_sample,
            delete_sample,
            get_all_samples,
            get_sample_by_id,
            update_sample,
            get_config,
            set_config,
            delete_config,
            process_user_question,
            cancel_user_task,
            is_user_task_finished,
            get_user_task_logs,
            get_user_task_result,
            save_generated_file,
            is_file_exsited
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_log() {
    const LOG4RS_CONFIG: &str = include_str!("../log4rs.yaml");
    // 创建临时文件并写入配置
    let mut temp_file =
        NamedTempFile::with_suffix(".yaml").expect("Failed to create temporary log4rs config file");
    temp_file
        .write_all(LOG4RS_CONFIG.as_bytes())
        .expect("Failed to write log4rs config to temporary file");
    let temp_path = temp_file
        .path()
        .to_str()
        .expect("Temporary file path is not valid UTF-8");
    // 从临时文件初始化日志
    log4rs::init_file(temp_path, Default::default())
        .expect("Failed to initialize log4rs from config file");
}

#[derive(Debug, Deserialize)]
pub struct CodeFile {
    pub name: String,
    pub path: String,
    pub content: String,
}

#[tauri::command]
async fn get_file_system(path: &str) -> Result<Vec<Value>, String> {
    // Helper function to recursively walk the directory
    fn walk_dir(path: &Path, parent_id: Option<&PathBuf>) -> Result<Value, io::Error> {
        // Ensure the file name is valid and convert it to a string
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid file name"))?;

        // Construct the entry object with absolute path as ID
        let mut entry = json!({
            "id": path.to_string_lossy(), // Use absolute path as ID
            "label": file_name,
            "isFolder": path.is_dir(),
            "children": []
        });

        if let Some(pid) = parent_id {
            entry["parentId"] = json!(pid.to_string_lossy());
        }

        // If it's a directory, recursively process its children
        if path.is_dir() {
            let mut children = Vec::new();
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let parent_path = entry_path.parent().map(PathBuf::from);
                children.push(walk_dir(&entry_path, parent_path.as_ref())?);
            }
            entry["children"] = json!(children);
        }

        Ok(entry)
    }

    // Start walking the directory from the given path
    let root_path = Path::new(path);

    // Validate the input path
    if !root_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    if !root_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    match walk_dir(root_path, None) {
        Ok(root) => Ok(vec![root]),
        Err(e) => Err(format!("Error reading directory: {}", e)),
    }
}

#[tauri::command]
async fn process_user_question(request: CodeGenRequest) -> Result<String, String> {
    let code_gen_task = CodeGenTask::new(request);
    let task_id = match task::execute_task(code_gen_task).await {
        Ok(id) => id,
        Err(e) => return Err(format!("{}", e)),
    };
    Ok(task_id)
}

#[tauri::command]
async fn cancel_user_task(task_id: String) -> Result<(), String> {
    task::cancel_task(task_id).await.to_tauri_result()
}

#[tauri::command]
async fn is_user_task_finished(task_id: String) -> Result<bool, String> {
    Ok(task::is_task_finished(task_id).await)
}

#[tauri::command]
async fn get_user_task_logs(task_id: String) -> Result<Option<Vec<TaskLog>>, String> {
    Ok(task::get_task_logs(task_id).await)
}

#[tauri::command]
async fn get_user_task_result(task_id: String) -> Result<Option<TaskResult>, String> {
    Ok(task::get_task_result(task_id).await)
}

#[tauri::command]
async fn save_generated_file(file: CodeFile) -> Result<bool, String> {
    save_file(&file.path, &file.content).to_tauri_result()
}

#[tauri::command]
async fn is_file_exsited(file_path: &str) -> Result<bool, String> {
    Ok(file_existed(file_path))
}
