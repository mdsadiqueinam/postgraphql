use std::collections::HashMap;
use deadpool_postgres::Client;
use tokio_postgres::Error;

use super::constant_queries::{TABLE_QUERY, COLUMN_QUERY};
use crate::entities::{column::{Column}, table::*};

/// Fetch table info for the given schema
async fn get_table_info(client: &Client, schema: &[&str]) -> Result<Vec<Table>, Error> {
    let rows = client.query(TABLE_QUERY, &[&schema]).await?;
    Ok(rows.into_iter().map(|row| Table::from_row(&row)).collect())
}

async fn check_and_create_relation(column: &Column, tables: &mut [Table], table_map: &HashMap<String, usize>) -> Result<(), std::io::Error> {
    // Find the table that the column belongs to
    if (column.is_foreign_key) {
        let foreign_table_name = column.foreign_table_name.as_ref().ok_or_else(|| {
            std::io::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Foreign table name is missing"))
        })?;
        let foreign_column_name = column.foreign_column_name.as_ref().ok_or_else(|| {
            std::io::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Foreign column name is missing"))
        })?;
    }
    Ok(())
}

/// Map columns into the tables by mutating the given `tables`
async fn map_column_info<'a>(
    client: &Client,
    schema: &[&str],
    tables: &mut [Table],
) -> Result<(), Error> {
    // Step 1: Get all columns
    let rows = client.query(COLUMN_QUERY, &[&schema]).await?;

    // Step 2: Borrow keys instead of cloning
    let table_map: HashMap<String, usize> = tables
        .iter()
        .enumerate()
        .map(|(i, t)| (t.name.clone(), i))
        .collect();

    // Step 3: Populate column info
    for row in rows {
        let table_name: &str = row.get("table_name");
        if let Some(index) = table_map.get(table_name) {
            let table = &mut tables[*index];
            let column = Column::from_row(&row);
            table.add_column(column);
        }
    }
    Ok(())
}

/// Fetch both tables and columns for the given schema
pub async fn get_schema_info(client: &Client, schema: &[&str]) -> Result<Vec<Table>, Error> {
    // Step 1: Get all tables
    let mut tables = get_table_info(client, schema).await?;

    // Step 2: Populate column info
    map_column_info(client, schema, &mut tables).await?;

    Ok(tables)
}
