use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{DataServiceError, DB_POOL};

// 数据结构定义
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    #[serde(rename = "dbType")]
    pub db_type: String,
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub database: String,
    pub extra_params: Option<String>,
}

// 数据源服务

#[tauri::command]
// 创建数据源
pub async fn create_ds(ds: DataSource) -> Result<String, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let id = sqlx::query_scalar(
        r#"INSERT INTO data_source 
            (id,name, db_type, host, port, username, password, database, extra_params)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id"#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&ds.name)
    .bind(&ds.db_type)
    .bind(&ds.host)
    .bind(ds.port)
    .bind(&ds.username)
    .bind(&ds.password)
    .bind(&ds.database)
    .bind(&ds.extra_params)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

#[tauri::command]
// 根据ID获取数据源
pub async fn get_ds_by_id(id: String) -> Result<DataSource, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let ds = sqlx::query_as::<_, DataSource>("SELECT * FROM data_source WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(ds)
}

#[tauri::command]
// 获取全部数据源
pub async fn get_all_ds() -> Result<Vec<DataSource>, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let list = sqlx::query_as::<_, DataSource>("SELECT * FROM data_source")
        .fetch_all(pool)
        .await?;
    Ok(list)
}

#[tauri::command]
// 更新数据源
pub async fn update_ds(ds: DataSource) -> Result<bool, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;

    let rows_affected = sqlx::query(
        r#"UPDATE data_source SET
            name = $1,
            db_type = $2,
            host = $3,
            port = $4,
            username = $5,
            password = $6,
            database = $7,
            extra_params = $8
            WHERE id = $9"#,
    )
    .bind(&ds.name)
    .bind(&ds.db_type)
    .bind(&ds.host)
    .bind(ds.port)
    .bind(&ds.username)
    .bind(&ds.password)
    .bind(&ds.database)
    .bind(&ds.extra_params)
    .bind(ds.id)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(rows_affected > 0)
}

#[tauri::command]
// 删除数据源
pub async fn delete_ds(id: String) -> Result<bool, DataServiceError> {
    let pool = DB_POOL.get().context("DB not initialized")?;
    let rows_affected = sqlx::query("DELETE FROM data_source WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(rows_affected > 0)
}
