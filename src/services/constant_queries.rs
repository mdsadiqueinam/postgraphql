pub(crate) const COLUMN_QUERY: &str = "
    SELECT
    col.table_schema,
    col.table_name,
    col.column_name,
    col.ordinal_position,
    col.column_default,
    col.is_nullable,
    col.data_type,
    col.character_maximum_length,
    col.character_octet_length,
    col.character_set_catalog,
    col.character_set_schema,
    col.character_set_name,
    col.numeric_precision,
    col.numeric_precision_radix,
    col.numeric_scale,
    col.datetime_precision,
    col.interval_type,
    col.interval_precision,
    col.identity_generation,
    col.identity_start,
    col.identity_increment,
    col.identity_maximum,
    col.identity_minimum,
    col.identity_cycle,
    col.is_generated,
    d.description AS comment,
    -- booleans from array_agg
    bool_or(con.contype = 'p') AS is_primary_key,
    bool_or(con.contype = 'u' OR con.contype = 'p') AS is_unique,
    bool_or(con.contype = 'f') AS is_foreign_key,
    -- foreign key target (may have multiple, but usually one)
    max(n_ref.nspname) AS foreign_table_schema,
    max(c_ref.relname) AS foreign_table_name,
    max(pa.attname) AS foreign_column_name
FROM information_schema.columns col
         JOIN pg_class c
              ON c.relname = col.table_name
         JOIN pg_namespace n
              ON n.oid = c.relnamespace
                  AND n.nspname = col.table_schema
         JOIN pg_attribute a
              ON a.attrelid = c.oid
                  AND a.attname = col.column_name
         LEFT JOIN pg_description d
                   ON d.objoid = c.oid
                       AND d.objsubid = a.attnum
         LEFT JOIN pg_constraint con
                   ON con.conrelid = c.oid
                       AND a.attnum = ANY (con.conkey)
         LEFT JOIN pg_class c_ref
                   ON c_ref.oid = con.confrelid
         LEFT JOIN pg_namespace n_ref
                   ON n_ref.oid = c_ref.relnamespace
         LEFT JOIN pg_attribute pa
                   ON pa.attrelid = c_ref.oid
                       AND pa.attnum = ANY (con.confkey)
WHERE col.table_schema = ANY($1)
GROUP BY
    col.table_schema,
    col.table_name,
    col.column_name,
    col.ordinal_position,
    col.column_default,
    col.is_nullable,
    col.data_type,
    col.character_maximum_length,
    col.character_octet_length,
    col.character_set_catalog,
    col.character_set_schema,
    col.character_set_name,
    col.numeric_precision,
    col.numeric_precision_radix,
    col.numeric_scale,
    col.datetime_precision,
    col.interval_type,
    col.interval_precision,
    col.identity_generation,
    col.identity_start,
    col.identity_increment,
    col.identity_maximum,
    col.identity_minimum,
    col.identity_cycle,
    col.is_generated,
    d.description
ORDER BY col.table_name, col.ordinal_position
";

pub(crate) const TABLE_QUERY: &str = "
    SELECT t.*, d.description AS comment
    FROM information_schema.tables t
            JOIN pg_class c
                ON c.relname = t.table_name
            JOIN pg_namespace n
                ON n.oid = c.relnamespace
                    AND n.nspname = t.table_schema
            LEFT JOIN pg_description d
                    ON d.objoid = c.oid
                        AND d.objsubid = 0  -- subid = 0 means it's a comment on the table itself
    WHERE t.table_schema = ANY($1)
    ORDER BY t.table_name
";
