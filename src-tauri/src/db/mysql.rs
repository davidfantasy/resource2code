use super::{DataSource, DatabaseOps};
use anyhow::Result;
use log::error;
use sqlx::{MySqlPool, Row};

pub struct MySQLOps;

#[async_trait::async_trait]
impl DatabaseOps for MySQLOps {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>> {
        let connection_url = format!(
            "mysql://{}:{}@{}:{}/{}?ssl-mode=disabled{}",
            ds.username,
            ds.password,
            ds.host,
            ds.port,
            ds.database,
            if ds.extra_params.is_some() {
                format!("&{}", ds.extra_params.unwrap())
            } else {
                String::new()
            }
        );

        let pool = MySqlPool::connect(&connection_url)
            .await
            .map_err(|e| anyhow::anyhow!("Connection failed: {}", e))?;

        let query = r#"
            SELECT TABLE_NAME 
            FROM INFORMATION_SCHEMA.TABLES 
            WHERE TABLE_SCHEMA = ? 
        "#;

        let tables: Vec<String> = sqlx::query(query)
            .bind(&ds.database)
            .fetch_all(&pool)
            .await?
            .iter()
            .filter_map(|row| row.try_get::<String, _>("TABLE_NAME").ok())
            .collect();

        Ok(tables)
    }

    async fn get_table_schema(&self, ds: DataSource, table_name: String) -> Result<String> {
        let connection_url = format!(
            "mysql://{}:{}@{}:{}/{}?ssl-mode=disabled{}",
            ds.username,
            ds.password,
            ds.host,
            ds.port,
            ds.database,
            if ds.extra_params.is_some() {
                format!("&{}", ds.extra_params.unwrap())
            } else {
                String::new()
            }
        );

        let pool = MySqlPool::connect(&connection_url).await.map_err(|e| {
            error!("Failed to connect to database: {}", e);
            anyhow::anyhow!("Failed to create MySQL connection pool: {}", e)
        })?;

        let schema: Option<String> = sqlx::query(&format!("SHOW CREATE TABLE `{}`", table_name))
            .map(|row: sqlx::mysql::MySqlRow| row.get::<String, _>(1))
            .fetch_optional(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("查询数据表结构失败: {}", e))?;
        Ok(schema.unwrap_or_else(|| "Table not found".to_string()))
    }
}
