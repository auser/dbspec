use sqlx::{MySqlPool, Row};

use super::{TableColumnDefinition, TableDefinition};

pub async fn introspect_mysql_tables(
    pool: MySqlPool,
) -> Result<Vec<TableDefinition>, anyhow::Error> {
    let what_db = sqlx::query("SELECT DATABASE() AS db")
        .fetch_one(&pool)
        .await?;
    let database_name = what_db.get::<String, &str>("db");

    let query = r#"
    SELECT 
        TABLE_NAME, TABLE_SCHEMA, TABLE_TYPE
    FROM 
        INFORMATION_SCHEMA.TABLES
    WHERE 
        TABLE_SCHEMA = ?
    "#;

    let res = sqlx::query(query)
        .bind(database_name)
        .fetch_all(&pool)
        .await?;

    let mut defs = vec![];
    for row in res.iter() {
        let table_name: String = row.get("TABLE_NAME");
        let table_schema: String = row.get("TABLE_SCHEMA");
        let column_definitions: Vec<TableColumnDefinition> =
            introspect_mysql_table(&pool, &table_schema).await?;
        let def = TableDefinition {
            table_name,
            table_schema,
            table_type: row.get("TABLE_TYPE"),
            column_definitions,
        };
        defs.push(def);
    }

    Ok(defs)
}

async fn introspect_mysql_table(
    connection: &MySqlPool,
    schema: &str,
) -> Result<Vec<TableColumnDefinition>, anyhow::Error> {
    let query = r#"
        SELECT
            TABLE_NAME, COLUMN_NAME, IS_NULLABLE, DATA_TYPE
        FROM INFORMATION_SCHEMA.COLUMNS
        WHERE
            TABLE_SCHEMA = ?
        ORDER BY
            TABLE_NAME, COLUMN_NAME
    "#;

    let res = sqlx::query(query)
        .bind(schema)
        .fetch_all(connection)
        .await?
        .iter()
        .map(|row| TableColumnDefinition {
            table_name: row.get("TABLE_NAME"),
            column_name: row.get("COLUMN_NAME"),
            data_type: row.get("DATA_TYPE"),
            is_nullable: match row.get("IS_NULLABLE") {
                "YES" => true,
                "NO" => false,
                _ => panic!("Unknown value for is_nullable"),
            },
        })
        .collect::<Vec<TableColumnDefinition>>();
    Ok(res)
}

fn build_table_column_def() {}
