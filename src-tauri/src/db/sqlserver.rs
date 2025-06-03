use super::{DataSource, DatabaseOps};
use anyhow::Result;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub struct SqlServerOps;

#[async_trait::async_trait]
impl DatabaseOps for SqlServerOps {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>> {
        let mut config = Config::new();

        config.host(ds.host);
        config.port(ds.port as u16);
        config.database(&ds.database);
        config.authentication(tiberius::AuthMethod::sql_server(ds.username, ds.password));

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE' AND TABLE_CATALOG = ?";

        let stream = client.query(query, &[&ds.database]).await?;
        let tables = stream
            .into_first_result()
            .await?
            .into_iter()
            .filter_map(|row| row.get::<&str, _>(0).map(|s| s.to_string()))
            .collect();

        Ok(tables)
    }

    async fn get_table_schema(&self, ds: DataSource, table_name: String) -> Result<String> {
        let mut config = Config::new();

        config.host(ds.host);
        config.port(ds.port as u16);
        config.database(ds.database);
        config.authentication(tiberius::AuthMethod::sql_server(ds.username, ds.password));

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let query = format!("SELECT OBJECT_DEFINITION(OBJECT_ID('{}'))", table_name);

        let schema = client
            .query(query, &[])
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .next()
            .and_then(|row| row.get::<&str, _>(0).map(|s| s.to_string()))
            .ok_or_else(|| anyhow::anyhow!("Table not found"))?;

        Ok(schema)
    }
}
