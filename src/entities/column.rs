use serde::{Serialize, Deserialize};

/// Text-based types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextDataType {
    pub maximum: Option<i32>,
    pub octet_length: Option<i32>,
    pub set_catalog: Option<String>,
    pub set_schema: Option<String>,
    pub set_name: Option<String>,
}

impl TextDataType {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            maximum: row.get("character_maximum_length"),
            octet_length: row.get("character_octet_length"),
            set_catalog: row.get("character_set_catalog"),
            set_schema: row.get("character_set_schema"),
            set_name: row.get("character_set_name"),
        }
    }
}

/// Numeric-based types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NumericDataType {
    pub precision: Option<i32>,
    pub scale: Option<i32>,
    pub radix: Option<i32>,
}

impl NumericDataType {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            precision: row.get("numeric_precision"),
            scale: row.get("numeric_scale"),
            radix: row.get("numeric_precision_radix"),
        }
    }
}

/// Temporal (date/time/interval) types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemporalDataType {
    pub datetime_precision: Option<i32>,
    pub interval_type: Option<String>,
    pub interval_precision: Option<i32>,
}

impl TemporalDataType {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            datetime_precision: row.get("datetime_precision"),
            interval_type: row.get("interval_type"),
            interval_precision: row.get("interval_precision"),
        }
    }
}

/// Identity/serial metadata
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityDataType {
    pub generation: Option<String>, // "ALWAYS" / "BY DEFAULT"
    pub start: Option<String>,
    pub increment: Option<String>,
    pub maximum: Option<String>,
    pub minimum: Option<String>,
    pub cycle: bool,
}

impl IdentityDataType {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Self {
            generation: row.get("identity_generation"),
            start: row.get("identity_start"),
            increment: row.get("identity_increment"),
            maximum: row.get("identity_maximum"),
            minimum: row.get("identity_minimum"),
            cycle: row.get("identity_cycle"),
        }
    }
}

/// Supported column data types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColumnDataType {
    // Text-like
    Text(TextDataType),       // varchar, text, char, name

    // Numbers
    SmallInt,                 // smallint
    Integer,                  // int4
    BigInt,                   // int8
    Numeric(NumericDataType), // numeric / decimal
    Real,                     // float4
    DoublePrecision,          // float8
    Oid,                      // oid
    Xid,                      // transaction id

    // Boolean
    Boolean,                  // bool

    // Temporal
    Date,                     // date
    TimestampWithTimeZone(TemporalDataType),
    Interval(TemporalDataType),

    // Network / system
    Inet,                     // inet
    Uuid,                     // uuid
    RegProc,                  // regproc
    RegType,                  // regtype

    // Binary
    Bytea,                    // bytea

    // Postgres internal
    PgLsn,
    PgDependencies,
    PgNodeTree,
    PgNdDistinct,
    PgMcvList,
    AnyArray,
    Array,

    // Fallback
    Other(String),
}

impl ColumnDataType {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        let data_type = row.get::<_, String>("data_type");
        match data_type.as_str() {
            "varchar" | "text" | "char" | "name" => Self::Text(TextDataType::from_row(row)),
            "smallint" => Self::SmallInt,
            "int4" => Self::Integer,
            "int8" => Self::BigInt,
            "numeric" | "decimal" => Self::Numeric(NumericDataType::from_row(row)),
            "float4" => Self::Real,
            "float8" => Self::DoublePrecision,
            "oid" => Self::Oid,
            "xid" => Self::Xid,
            "bool" => Self::Boolean,
            "date" => Self::Date,
            "timestamp with time zone" => Self::TimestampWithTimeZone(TemporalDataType::from_row(row)),
            "interval" => Self::Interval(TemporalDataType::from_row(row)),
            "inet" => Self::Inet,
            "uuid" => Self::Uuid,
            "regproc" => Self::RegProc,
            "regtype" => Self::RegType,
            "bytea" => Self::Bytea,
            "pg_lsn" => Self::PgLsn,
            "pg_dependencies" => Self::PgDependencies,
            "pg_node_tree" => Self::PgNodeTree,
            "pg_nd_distinct" => Self::PgNdDistinct,
            "pg_mcv_list" => Self::PgMcvList,
            "anyarray" => Self::AnyArray,
            "array" => Self::Array,
            _ => Self::Other(row.get("data_type")),
        }
    }
}

/// Column abstraction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Column {
    pub table_name: String,
    pub name: String,
    pub ordinal_position: i32,
    pub data_type: ColumnDataType,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub is_nullable: bool,
    pub is_generated: bool,
    pub is_foreign_key: bool,
    pub foreign_table_name: Option<String>,
    pub foreign_column_name: Option<String>,
    pub default_value: Option<String>,
    pub comment: Option<String>,
}

impl Column {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        let is_nullable: String = row.get("is_nullable");
        let is_primary_key: Option<bool> = row.get("is_primary_key");
        let is_unique: Option<bool> = row.get("is_unique");
        let is_foreign_key: Option<bool> = row.get("is_foreign_key");
        let is_generated: Option<String> = row.get("is_generated");
        
        Self {
            table_name: row.get("table_name"),
            name: row.get("column_name"),
            ordinal_position: row.get("ordinal_position"),
            data_type: ColumnDataType::from_row(row),
            is_primary_key: is_primary_key.unwrap_or(false),
            is_unique: is_unique.unwrap_or(false),
            is_nullable: is_nullable == "YES",
            is_generated: is_generated == Some("YES".into()),
            foreign_table_name: row.get("foreign_table_name"),
            foreign_column_name: row.get("foreign_column_name"),
            is_foreign_key: is_foreign_key.unwrap_or(false),
            default_value: row.get("column_default"),
            comment: row.get("comment"),
        }
    }
}