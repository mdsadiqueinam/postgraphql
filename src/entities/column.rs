use serde::{Serialize, Deserialize};

/// Text-based types
#[derive(Debug, Serialize, Deserialize)]
pub struct TextDataType {
    pub maximum: Option<i32>,
    pub octet_length: Option<i32>,
    pub set_catalog: Option<String>,
    pub set_schema: Option<String>,
    pub set_name: Option<String>,
}

/// Numeric-based types
#[derive(Debug, Serialize, Deserialize)]
pub struct NumericDataType {
    pub precision: Option<i32>,
    pub scale: Option<i32>,
    pub radix: Option<i32>,
}

/// Temporal (date/time/interval) types
#[derive(Debug, Serialize, Deserialize)]
pub struct TemporalDataType {
    pub datetime_precision: Option<i32>,
    pub interval_type: Option<String>,
    pub interval_precision: Option<i32>,
}

/// Identity/serial metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityDataType {
    pub generation: Option<String>, // "ALWAYS" / "BY DEFAULT"
    pub start: Option<String>,
    pub increment: Option<String>,
    pub maximum: Option<String>,
    pub minimum: Option<String>,
    pub cycle: bool,
}

/// Supported column data types
#[derive(Debug, Serialize, Deserialize)]
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

/// Column abstraction
#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub ordinal_position: i32,
    pub data_type: ColumnDataType,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub is_nullable: bool,
    pub foreign_table_name: Option<String>,
    pub foreign_column_name: Option<String>,
    pub is_foreign_key: bool,
    pub default_value: Option<String>,
    pub comment: Option<String>,
}
