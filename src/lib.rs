pub mod pool;
pub mod entities;
pub mod services;

// Re-export main public API
pub use pool::{PoolOrConfig, create_pool};
pub use services::{
    db_schema_service::get_schema_info,
    constant_queries::{TABLE_QUERY, COLUMN_QUERY}
};
pub use entities::{
    table::{Table, Relation, RelationType},
    column::{Column, ColumnDataType, TextDataType, NumericDataType, TemporalDataType, IdentityDataType}
};