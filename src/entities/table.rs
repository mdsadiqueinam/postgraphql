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

    fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    fn add_column_from_row(&mut self, row: &tokio_postgres::Row) {
        let column = Column::from_row(row);
        self.add_column(column);
    }
}