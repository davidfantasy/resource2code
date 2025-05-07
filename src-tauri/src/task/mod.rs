use anyhow::{anyhow, Result};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

pub mod code_gen_task;

#[derive(Debug, Clone, Serialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub enum TaskLogLevel {
    Warn,
    Info,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskLog {
    pub timestamp: u64,
    pub message: String,
    pub level: TaskLogLevel,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskGenFile {
    pub name: String,
    pub path: Option<PathBuf>,
    pub content: String,
}

impl TaskLog {
    pub fn new<S: Into<String>>(message: S, level: TaskLogLevel) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(std::time::Duration::ZERO)
                .as_millis() as u64,
            message: message.into(),
            level,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum TaskResult {
    /// 代码生成任务的结果
    CodeGen { files: Vec<TaskGenFile> },
    /// 无返回值的任务
    Empty,
}

#[async_trait]
pub trait Task: Send + Sync {
    async fn start(&mut self, sender: tokio::sync::mpsc::Sender<TaskLog>) -> Result<TaskResult>;
    async fn cancel(&mut self) -> Result<()>;
}

struct TaskInfo {
    status: TaskStatus,
    logs: Vec<TaskLog>,
    task_result: TaskResult,
}

static TASKS: Lazy<tokio::sync::RwLock<HashMap<String, Box<dyn Task>>>> =
    Lazy::new(|| tokio::sync::RwLock::new(HashMap::new()));

static TASK_INFOS: Lazy<tokio::sync::RwLock<HashMap<String, TaskInfo>>> =
    Lazy::new(|| tokio::sync::RwLock::new(HashMap::new()));

pub async fn execute_task(task: impl Task + 'static) -> Result<String> {
    let task_id = Uuid::new_v4().to_string();
    // 使用异步通道并设置缓冲区大小
    let (log_sender, mut log_receiver) = tokio::sync::mpsc::channel(100);

    let task_status = TaskInfo {
        status: TaskStatus::Pending,
        logs: Vec::new(),
        task_result: TaskResult::Empty,
    };

    //插入任务
    TASKS.write().await.insert(task_id.clone(), Box::new(task));
    TASK_INFOS
        .write()
        .await
        .insert(task_id.clone(), task_status);

    let task_id_clone = task_id.clone();

    //异步接受任务日志并更新
    tokio::spawn(async move {
        while let Some(log) = log_receiver.recv().await {
            let mut task_infos = TASK_INFOS.write().await;
            if let Some(info) = task_infos.get_mut(&task_id_clone) {
                info.logs.push(log);
            }
        }
    });

    //异步启动任务
    let task_id_clone = task_id.clone();
    tokio::spawn(async move {
        let mut tasks = TASKS.write().await;
        if let Some(task) = tasks.get_mut(&task_id_clone) {
            update_task_status(&task_id_clone, TaskStatus::Running).await;
            match task.start(log_sender.clone()).await {
                Ok(result) => {
                    update_task_status(&task_id_clone, TaskStatus::Completed).await;
                    update_task_result(&task_id_clone, result).await;
                    log_sender
                        .send(TaskLog::new("任务执行完成", TaskLogLevel::Info))
                        .await
                        .unwrap();
                }
                Err(e) => {
                    update_task_status(&task_id_clone, TaskStatus::Failed).await;
                    log_sender
                        .send(TaskLog::new(
                            format!("任务执行失败: {}", e),
                            TaskLogLevel::Error,
                        ))
                        .await
                        .unwrap();
                }
            }
        }
    });
    Ok(task_id)
}

pub async fn cancel_task(task_id: String) -> Result<()> {
    let mut tasks_mutex = TASKS.write().await;
    let task = tasks_mutex
        .get_mut(&task_id)
        .ok_or_else(|| anyhow!("Task not found"))?;
    update_task_status(&task_id, TaskStatus::Cancelled).await;
    task.cancel().await?;
    Ok(())
}

pub async fn is_task_finished(task_id: String) -> bool {
    let guard = TASK_INFOS.read().await;
    guard.get(&task_id).map_or(false, |task| {
        matches!(
            task.status,
            TaskStatus::Completed | TaskStatus::Cancelled | TaskStatus::Failed
        )
    })
}

pub async fn get_task_logs(task_id: String) -> Option<Vec<TaskLog>> {
    TASK_INFOS
        .read()
        .await
        .get(&task_id)
        .map(|task| task.logs.clone())
}

pub async fn get_task_result(task_id: String) -> Option<TaskResult> {
    TASK_INFOS
        .read()
        .await
        .get(&task_id)
        .map(|task| task.task_result.clone())
}

pub fn periodic_cleanup_inactive_tasks(interval_seconds: u64) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(interval_seconds)).await;
            let mut task_infos = TASK_INFOS.write().await;
            task_infos.retain(|_, wrapper| {
                !matches!(
                    wrapper.status,
                    TaskStatus::Completed | TaskStatus::Cancelled | TaskStatus::Failed
                )
            });
        }
    });
}

async fn update_task_status(task_id: &str, status: TaskStatus) {
    if let Some(info) = TASK_INFOS.write().await.get_mut(task_id) {
        info.status = status;
    }
}

async fn update_task_result(task_id: &str, result: TaskResult) {
    if let Some(info) = TASK_INFOS.write().await.get_mut(task_id) {
        info.task_result = result;
    }
}
