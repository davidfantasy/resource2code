use sqlx::mysql::MySqlPool;
use sqlx::Row;

use crate::storage::datasource::DataSource;

#[tauri::command]
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
