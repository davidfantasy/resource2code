use super::{DataSource, DatabaseOps};
use anyhow::Result;
use log::error;
use sqlx::{Row, SqlitePool};

pub struct SqliteOps;

#[async_trait::async_trait]
impl DatabaseOps for SqliteOps {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>> {
        let url = format!(
            "sqlite://{}{}",
            ds.database,
            if ds.extra_params.is_some() {
                format!("?{}", ds.extra_params.unwrap())
            } else {
                String::new()
            }
        );
        let pool = SqlitePool::connect(&url)
            .await
            .map_err(|e| anyhow::anyhow!("Connection failed: {}", e))?;

        let query = r#"
            SELECT name 
            FROM sqlite_master 
            WHERE type = 'table' AND name NOT LIKE 'sqlite_%'
        "#;

        let tables: Vec<String> = sqlx::query(query)
            .fetch_all(&pool)
            .await?
            .iter()
            .filter_map(|row| row.try_get::<String, _>("name").ok())
            .collect();

        Ok(tables)
    }

    async fn get_table_schema(&self, ds: DataSource, table_name: String) -> Result<String> {
        let url = format!(
            "sqlite://{}{}",
            ds.database,
            if !ds.extra_params.is_none() {
                format!("?{}", ds.extra_params.unwrap())
            } else {
                String::new()
            }
        );
        let pool = SqlitePool::connect(&url).await.map_err(|e| {
            error!("Failed to connect to database: {}", e);
            anyhow::anyhow!("Failed to create SQLite connection pool: {}", e)
        })?;
        let schema: Option<String> = sqlx::query("SELECT sql FROM sqlite_master WHERE name = ?")
            .bind(&table_name)
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<String, _>("sql"))
            .fetch_optional(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("查询数据表结构失败: {}", e))?;
        Ok(schema.unwrap_or_else(|| "Table not found".to_string()))
    }
}
