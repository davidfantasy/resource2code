use super::{DataSource, DatabaseOps};
use anyhow::Result;
use clickhouse::Client;

pub struct ClickHouseOps;

#[async_trait::async_trait]
impl DatabaseOps for ClickHouseOps {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>> {
        let client = Client::default()
            .with_url(format!("http://{}:{}", ds.host, ds.port))
            .with_user(ds.username)
            .with_password(ds.password)
            .with_database(ds.database.clone());
        let tables = client
            .query("select name from system.tables where database = ?")
            .bind(ds.database)
            .fetch_all::<String>()
            .await?;
        Ok(tables)
    }

    async fn get_table_schema(&self, ds: DataSource, table_name: String) -> Result<String> {
        let client = Client::default()
            .with_url(format!("http://{}:{}", ds.host, ds.port))
            .with_user(ds.username)
            .with_password(ds.password)
            .with_database(ds.database.clone());
        let schemas = client
            .query(format!("show create table {}.{}", ds.database, table_name).as_str())
            .fetch_all::<String>()
            .await?;
        if schemas.is_empty() {
            Err(anyhow::anyhow!("Table not found"))
        } else {
            Ok(schemas[0].clone())
        }
    }
}
