use serde::{Deserialize, Serialize};

use super::column::Column;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Relation {
    pub table: Table,
    pub column: Column,
    pub relation_type: RelationType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            schema: row.get("table_schema"),
            name: row.get("table_name"),
            r#type: row.get("table_type"),
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