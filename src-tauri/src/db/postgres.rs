use super::{DataSource, DatabaseOps};
use anyhow::Context;
use anyhow::Result;
use log::error;
use sqlx::{PgPool, Row};

pub struct PostgresOps;

#[async_trait::async_trait]
impl DatabaseOps for PostgresOps {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            ds.username,
            urlencoding::encode(&ds.password),
            ds.host,
            ds.port,
            ds.database,
            if ds.extra_params.is_some() {
                format!("?{}", ds.extra_params.unwrap())
            } else {
                String::new()
            }
        );

        let pool = PgPool::connect(&url)
            .await
            .map_err(|e| anyhow::anyhow!("Connection failed: {}", e))?;

        let query = r#"
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_catalog = $1
        "#;

        let tables: Vec<String> = sqlx::query(query)
            .bind(&ds.database)
            .fetch_all(&pool)
            .await?
            .iter()
            .filter_map(|row| row.try_get::<String, _>("table_name").ok())
            .collect();

        Ok(tables)
    }

    async fn get_table_schema(&self, ds: DataSource, table_name: String) -> Result<String> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            ds.username,
            urlencoding::encode(&ds.password),
            ds.host,
            ds.port,
            ds.database,
            if ds.extra_params.is_some() {
                format!("?{}", ds.extra_params.unwrap())
            } else {
                String::new()
            }
        );

        let pool = PgPool::connect(&url).await.map_err(|e| {
            error!("Failed to connect to database: {}", e);
            anyhow::anyhow!("Failed to create PostgreSQL connection pool: {}", e)
        })?;

        let schema: Option<String> =
            sqlx::query("SELECT pg_catalog.pg_get_tabledef('public', $1) as table_def")
                .bind(&table_name)
                .map(|row: sqlx::postgres::PgRow| row.get::<String, _>("table_def"))
                .fetch_optional(&pool)
                .await
                .map_err(|e| anyhow::anyhow!("查询数据表结构失败: {}", e))?;

        Ok(schema.unwrap_or_else(|| "Table not found".to_string()))
    }
}
