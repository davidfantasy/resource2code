use super::DataSource;
use anyhow::{anyhow, Result};

mod clickhouse;
mod mysql;
mod postgres;
mod sqlite;
mod sqlserver;

use clickhouse::ClickHouseOps;
use mysql::MySQLOps;
use postgres::PostgresOps;
use sqlite::SqliteOps;
use sqlserver::SqlServerOps;

#[async_trait::async_trait]
pub trait DatabaseOps: Send + Sync {
    async fn get_tables(&self, ds: DataSource) -> Result<Vec<String>>;
    async fn get_table_schema(&self, ds: DataSource, table_name: String) -> Result<String>;
}

pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
    SQLServer,
    ClickHouse,
}

pub async fn get_tables(ds: DataSource) -> Result<Vec<String>> {
    let ops = get_database_ops(&ds.db_type)?;
    ops.get_tables(ds).await
}

pub async fn get_table_schema(ds: DataSource, table_name: String) -> Result<String> {
    let ops = get_database_ops(&ds.db_type)?;
    ops.get_table_schema(ds, table_name).await
}

fn get_database_ops(db_type: &str) -> Result<Box<dyn DatabaseOps>> {
    match db_type {
        "clickhouse" => Ok(Box::new(ClickHouseOps)),
        "mysql" => Ok(Box::new(MySQLOps)),
        "postgres" => Ok(Box::new(PostgresOps)),
        "sqlite" => Ok(Box::new(SqliteOps)),
        "sqlserver" => Ok(Box::new(SqlServerOps)),
        _ => Err(anyhow!("Unsupported database type: {}", db_type)),
    }
}
