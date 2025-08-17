use super::column::Column;

pub struct Table {
    schema: String,
    name: String,
    r#type: String,
    columns: Vec<Column>,
}
