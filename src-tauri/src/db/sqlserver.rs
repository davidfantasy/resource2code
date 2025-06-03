use super::{DataSource, DatabaseOps};
use anyhow::Result;
use tiberius::{Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub struct SqlServerOps;

const TABLE_DDL_SQL: &str = r#"
DECLARE @TableName NVARCHAR(128) = '{tableName}';
DECLARE @SchemaName NVARCHAR(128) = '{schemaName}'; 
-- 生成列定义
SELECT 
    'CREATE TABLE [' + @SchemaName + '].[' + @TableName + '] (' + CHAR(13) +
    STRING_AGG(
        '    [' + c.name + '] ' + 
        UPPER(t.name) + 
        CASE 
            WHEN t.name IN ('varchar','nvarchar','char','nchar','varbinary') 
                THEN '(' + IIF(c.max_length = -1, 'MAX', CAST(c.max_length AS VARCHAR)) + ')' 
            WHEN t.name IN ('decimal','numeric') 
                THEN '(' + CAST(c.precision AS VARCHAR) + ',' + CAST(c.scale AS VARCHAR) + ')' 
            ELSE '' 
        END +
        ' ' + 
        IIF(c.is_nullable = 0, 'NOT NULL', 'NULL') +
        IIF(ic.column_id IS NOT NULL, ' IDENTITY(1,1)', ''),
        ',' + CHAR(13)
    ) 
    + CHAR(13) + ');'
FROM sys.columns c
JOIN sys.types t ON c.user_type_id = t.user_type_id
LEFT JOIN sys.identity_columns ic ON c.object_id = ic.object_id AND c.column_id = ic.column_id
WHERE c.object_id = OBJECT_ID(@SchemaName + '.' + @TableName);
"#;

#[async_trait::async_trait]
impl DatabaseOps for SqlServerOps {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>> {
        let mut config = Config::new();

        config.host(ds.host);
        config.port(ds.port as u16);
        config.database(&ds.database);
        config.authentication(tiberius::AuthMethod::sql_server(ds.username, ds.password));
        config.trust_cert();

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let select = Query::new(format!("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE' AND TABLE_CATALOG = '{}'", ds.database));
        let stream = select.query(&mut client).await?;
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
        config.trust_cert();

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let mut client = Client::connect(config, tcp.compat_write()).await?;

        let sql = TABLE_DDL_SQL
            .replace("{schemaName}", "dbo")
            .replace("{tableName}", &table_name);
        let select = Query::new(sql);
        let stream = select.query(&mut client).await?;

        let schema = stream
            .into_first_result()
            .await?
            .into_iter()
            .next()
            .and_then(|row| row.get::<&str, _>(0).map(|s| s.to_string()))
            .ok_or_else(|| anyhow::anyhow!("Table not found"))?;

        Ok(schema)
    }
}
