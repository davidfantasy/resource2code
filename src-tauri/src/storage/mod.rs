use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

pub mod code_sample;
pub mod datasource;
pub mod sys_config;

// 错误类型定义
#[derive(Debug, thiserror::Error, Serialize)]
pub enum DataServiceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Not Found Data")]
    NotFound,
    #[error("Other Error: {0}")]
    Other(String),
}

impl From<sqlx::Error> for DataServiceError {
    fn from(err: sqlx::Error) -> Self {
        DataServiceError::DatabaseError(err.to_string())
    }
}

impl From<anyhow::Error> for DataServiceError {
    fn from(error: anyhow::Error) -> Self {
        DataServiceError::Other(error.to_string())
    }
}
// 初始化数据库连接池
pub async fn init_db() -> Result<()> {
    let db_exists = std::path::Path::new("resource2code.db").exists();
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite://resource2code.db?cache=shared&mode=rwc")
        .await?;
    if !db_exists {
        let init_sql = include_str!("init.sql");
        sqlx::query(init_sql).execute(&pool).await?;
    }
    // 将连接池存储在全局 OnceCell 中
    DB_POOL.set(pool).map_err(|_| "Failed to set pool").unwrap();
    Ok(())
}
