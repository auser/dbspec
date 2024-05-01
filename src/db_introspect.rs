#![allow(dead_code)]

use sqlx::{MySqlPool, PgPool};
mod mysql;
mod pg;
use mysql::introspect_mysql_tables;
use pg::introspect_pg_tables;

#[derive(Debug)]
pub struct TableDefinition {
    pub table_name: String,
    pub table_schema: String,
    pub table_type: String,
    pub column_definitions: Vec<TableColumnDefinition>,
}

#[derive(Debug)]
pub struct TableColumnDefinition {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: bool,
}

pub async fn get_tables(connection_string: &str) -> Result<Vec<TableDefinition>, anyhow::Error> {
    let tables = get_table_definitions(connection_string).await?;
    Ok(tables)
}

pub(crate) async fn get_table_definitions(
    connection_string: &str,
) -> Result<Vec<TableDefinition>, anyhow::Error> {
    if connection_string.starts_with("postgres") {
        let pool = PgPool::connect(connection_string).await.unwrap();
        introspect_pg_tables(pool).await
    } else if connection_string.starts_with("mysql") {
        let connection = MySqlPool::connect(connection_string).await.unwrap();
        introspect_mysql_tables(connection).await
    } else {
        panic!("Unsupported database");
    }
}
