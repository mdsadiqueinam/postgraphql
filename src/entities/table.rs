use super::column::Column;

pub struct Table {
    schema: String,
    name: String,
    r#type: String,
    columns: Vec<Column>,
}

impl Table {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            schema: row.get("schema"),
            name: row.get("name"),
            r#type: row.get("type"),
            columns: Vec::new(),
        }
    }
}