use super::column::Column;

pub struct Table {
    pub schema: String,
    pub name: String,
    pub r#type: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub(crate) fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            schema: row.get("schema"),
            name: row.get("name"),
            r#type: row.get("type"),
            columns: Vec::new(),
        }
    }

    pub(crate) fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub(crate) fn add_column_from_row(&mut self, row: &tokio_postgres::Row) {
        let column = Column::from_row(row);
        self.add_column(column);
    }
}