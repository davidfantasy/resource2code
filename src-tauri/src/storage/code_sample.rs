use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{DataServiceError, DB_POOL};

// 数据结构定义
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CodeSample {
    pub id: String,
    pub name: String,
    pub content: String,
}

#[tauri::command]
// 创建 CodeStandard
pub async fn create_sample(cs: CodeSample) -> Result<String, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;

    let id = sqlx::query_scalar(
        r#"INSERT INTO code_sample 
            (id,name, content)
            VALUES ($1, $2, $3)
            RETURNING id"#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&cs.name)
    .bind(&cs.content)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

#[tauri::command]
// 根据ID获取 CodeStandard
pub async fn get_sample_by_id(id: &str) -> Result<CodeSample, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;

    let cs = sqlx::query_as::<_, CodeSample>("SELECT * FROM code_sample WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(cs)
}

#[tauri::command]
// 获取全部 CodeStandard
pub async fn get_all_samples() -> Result<Vec<CodeSample>, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let list = sqlx::query_as::<_, CodeSample>("SELECT * FROM code_sample")
        .fetch_all(pool)
        .await?;
    Ok(list)
}

#[tauri::command]
// 更新 CodeStandard
pub async fn update_sample(cs: CodeSample) -> Result<(), DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    sqlx::query(
        r#"UPDATE code_sample SET
            name = $1,
            content = $2
            WHERE id = $3"#,
    )
    .bind(&cs.name)
    .bind(&cs.content)
    .bind(&cs.id)
    .execute(pool)
    .await?;
    Ok(())
}

#[tauri::command]
// 删除 CodeStandard
pub async fn delete_sample(id: String) -> Result<bool, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let rows_affected = sqlx::query("DELETE FROM code_sample WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected > 0)
}
