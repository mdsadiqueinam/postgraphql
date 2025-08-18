use super::column::Column;

pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

pub struct Relation {
    pub table: Table,
    pub column: Column,
    pub relation_type: RelationType,
}

pub struct Table {
    pub schema: String,
    pub name: String,
    pub r#type: String,
    pub columns: Vec<Column>,
    pub relations: Vec<Relation>,
}

impl Table {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            schema: row.get("schema"),
            name: row.get("name"),
            r#type: row.get("type"),
            columns: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn add_relation(&mut self, relation: Relation) {
        self.relations.push(relation);
    }
}