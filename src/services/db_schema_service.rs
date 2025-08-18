use super::constant_queries::{TABLE_QUERY};
use postgraphql::entities::table::Table;

async fn get_table_info(client: &deadpool_postgres::Client, schema: &Vec<String>) {
    let rows = client.query(TABLE_QUERY, &[&schema]).await.unwrap();
    let tables = rows.iter().map(|row| Table::from_row(row)).collect::<Vec<_>>();
}   