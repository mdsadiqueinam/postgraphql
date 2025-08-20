use std::collections::HashMap;
use deadpool_postgres::Client;
use tokio_postgres::{Error};

use super::constant_queries::{TABLE_QUERY, COLUMN_QUERY};
use crate::entities::{column::{Column}, table::*};

/// Fetch table info for the given schema
async fn get_table_info(client: &Client, schema: &[&str]) -> Result<Vec<Table>, Error> {
    let rows = client.query(TABLE_QUERY, &[&schema]).await?;
    
    // Pre-allocate with known capacity to avoid reallocations
    let mut tables = Vec::with_capacity(rows.len());
    for row in rows {
        tables.push(Table::from_row(&row));
    }
    
    Ok(tables)
}

/// Map columns into the tables by mutating the given `tables`
/// Uses a more efficient approach with pre-allocated capacity and reduced string allocations
async fn map_column_info(
    client: &Client,
    schema: &[&str],
    tables: &mut [Table],
) -> Result<(), Error> {
    // Step 1: Get all columns
    let rows = client.query(COLUMN_QUERY, &[&schema]).await?;

    // Step 2: Create map with owned strings to avoid borrowing conflicts
    let table_map: HashMap<String, usize> = tables
        .iter()
        .enumerate()
        .map(|(i, t)| (t.name.clone(), i))
        .collect();

    // Step 3: Process each row and add columns to corresponding tables
    for row in rows {
        let table_name: String = row.get("table_name");
        if let Some(&index) = table_map.get(&table_name) {
            let column = Column::from_row(&row);
            tables[index].add_column(column);
        }
    }
    Ok(())
}

/// Fetch both tables and columns for the given schema(s)
/// Optimized to minimize database round trips and memory allocations
/// 
/// # Arguments
/// * `client` - Database client connection
/// * `schema` - Array of schema names to query (supports multiple schemas in one call)
/// 
/// # Returns
/// * `Result<Vec<Table>, Error>` - Vector of tables with populated columns or database error
/// 
pub async fn get_schema_info(client: &Client, schema: &[&str]) -> Result<Vec<Table>, Error> {
    // Early return for empty schema array - no database calls needed
    if schema.is_empty() {
        return Ok(Vec::new());
    }

    // Log the schemas being queried for debugging (optional)
    #[cfg(debug_assertions)]
    eprintln!("Querying schemas: {:?}", schema);

    // Step 1: Get all tables from all specified schemas in one query
    let mut tables = get_table_info(client, schema).await?;

    if !tables.is_empty() {
        // Step 2: Populate column info efficiently for all tables
        map_column_info(client, schema, &mut tables).await?;
    }

    Ok(tables)
}

#[cfg(test)]
mod tests {
    use crate::create_pool;

    const SCHEMA: &[&str] = &["public"];
    const DB_URL: &str = "postgres://postgres:Aa123456@localhost/app-success-co";

    #[tokio::test]
    async fn test_get_schema_info() {
       let pool = create_pool(crate::PoolOrConfig::DatabaseUrl(DB_URL.to_string()));
       let client = pool.get().await.unwrap();
       let result = super::get_schema_info(&client, SCHEMA).await;
       println!("Schema info: {:?}", result);
       assert!(result.is_ok());
    }
}