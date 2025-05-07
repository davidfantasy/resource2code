use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{DataServiceError, DB_POOL};

// 系统配置数据结构
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SysConfig {
    pub key: String,
    pub value: String,
}

#[tauri::command]
// 获取系统配置
pub async fn get_config(key: String) -> Result<Option<String>, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let config = sqlx::query_as::<_, SysConfig>("SELECT * FROM sys_config WHERE key = $1")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(config.map(|c| c.value))
}

#[tauri::command]
// 设置系统配置（存在则更新，不存在则插入）
pub async fn set_config(key: String, value: String) -> Result<bool, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;

    let rows_affected = sqlx::query(
        r#"INSERT INTO sys_config (key, value) 
           VALUES ($1, $2)
           ON CONFLICT(key) DO UPDATE SET value = $2"#,
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

#[tauri::command]
// 删除系统配置
pub async fn delete_config(key: String) -> Result<bool, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let rows_affected = sqlx::query("DELETE FROM sys_config WHERE key = $1")
        .bind(key)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected > 0)
}
