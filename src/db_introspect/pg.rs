use rayon::prelude::*;
use sqlx::{PgPool, Row};

use super::{TableColumnDefinition, TableDefinition};

pub(crate) async fn introspect_pg_tables(
    connection: PgPool,
) -> Result<Vec<TableDefinition>, anyhow::Error> {
    let query = r#"
    SELECT 
        table_name, table_type, table_schema
    FROM 
        information_schema.tables
    WHERE 
        table_schema = $1
    "#;

    let res = sqlx::query(query)
        .bind("public")
        .fetch_all(&connection)
        .await?;

    let mut defs = vec![];
    for row in res.iter() {
        let table_name: String = row.get("table_name");
        let table_schema: String = row.get("table_schema");
        let column_definitions: Vec<TableColumnDefinition> =
            introspect_pg_table(&connection, &table_name, &table_schema).await?;
        let def = TableDefinition {
            table_name,
            table_schema,
            table_type: row.get("table_type"),
            column_definitions,
        };
        defs.push(def);
    }

    Ok(defs)
}

pub(crate) async fn introspect_pg_table(
    connection: &PgPool,
    table_name: &str,
    table_schema: &str,
) -> Result<Vec<TableColumnDefinition>, anyhow::Error> {
    let query = r#"
    SELECT 
        table_name, column_name, is_nullable, data_type
    FROM INFORMATION_SCHEMA.COLUMNS
    WHERE 
        table_name = $1
        AND
        table_schema = $2
    ORDER BY 
        column_name
    "#;

    let res = sqlx::query(query)
        .bind(table_name)
        .bind(table_schema)
        .fetch_all(connection)
        .await?
        .par_iter()
        .map(|row| TableColumnDefinition {
            table_name: row.get("table_name"),
            column_name: row.get("column_name"),
            data_type: row.get("data_type"),
            is_nullable: match row.get("is_nullable") {
                "YES" => true,
                "NO" => false,
                _ => panic!("Unknown value for is_nullable"),
            },
        })
        .collect::<Vec<TableColumnDefinition>>();
    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_can_pull_tables_from_pg() {
        let connection = psql_pool().await;
        let tables = introspect_pg_tables(connection).await;
        assert!(tables.is_ok(), "Failed to introspect tables");
        assert!(tables.unwrap().len() > 0, "No tables found");
    }
    pub const TEST_POSTGRES_URL: &str = "postgresql://postgres:postgres@localhost:5433/production";

    pub async fn psql_pool() -> PgPool {
        PgPool::connect(TEST_POSTGRES_URL).await.unwrap()
    }
}
