use super::DataSource;
use anyhow::Context;
use anyhow::Result;
use log::error;
use sqlx::{MySqlPool, Row};

pub async fn get_tables(ds: DataSource) -> Result<Vec<String>, String> {
    // 确保数据库名称存在
    let database = ds.database.ok_or("Database name is required".to_string())?;
    // 构建 MySQL 连接字符串
    let connection_url = format!(
        "mysql://{}:{}@{}:{}/{}?ssl-mode=disabled",
        ds.username, ds.password, ds.host, ds.port, database
    );

    // 创建连接池
    let pool = MySqlPool::connect(&connection_url)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    // 查询当前数据库的所有表
    let query = r#"
        SELECT TABLE_NAME 
        FROM INFORMATION_SCHEMA.TABLES 
        WHERE TABLE_SCHEMA = ? 
    "#;

    let tables: Vec<String> = sqlx::query(query)
        .bind(&database)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?
        .iter()
        .filter_map(|row| row.try_get::<String, _>("TABLE_NAME").ok())
        .collect();

    Ok(tables)
}

pub async fn get_table_schema(ds: DataSource, table_name: String) -> Result<String> {
    // 构建数据库连接字符串
    let url = format!(
        "mysql://{}:{}@{}:{}/{}?ssl-mode=disabled",
        ds.username,
        urlencoding::encode(&ds.password), // 对密码进行URL编码
        ds.host,
        ds.port,
        ds.database.as_ref().context("Database name is required")? // 强制要求提供数据库名
    );
    // 创建数据库连接池
    let pool = MySqlPool::connect(&url).await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        anyhow::anyhow!("Failed to create MySQL connection pool: {}", e)
    })?;

    // 查询表的 schema
    let schema: Option<String> = sqlx::query(&format!("SHOW CREATE TABLE `{}`", table_name))
        .map(|row: sqlx::mysql::MySqlRow| row.get::<String, _>(1))
        .fetch_optional(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("查询数据表结构失败: {}", e))?;
    Ok(schema.unwrap_or_else(|| "Table not found".to_string()))
}
