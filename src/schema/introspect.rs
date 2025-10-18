use serde::{Deserialize, Serialize};
use crate::schema::{Column, ColumnType, Table, ForeignKey};

/// Serializable schema representation for SDK generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaExport {
    pub tables: Vec<TableSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
    pub foreign_keys: Vec<ForeignKeySchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
    pub typescript_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub unique: bool,
    pub auto_increment: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeySchema {
    pub column: String,
    pub references_table: String,
    pub references_column: String,
}

impl TableSchema {
    /// Convert a Table to a serializable TableSchema
    pub fn from_table(table: &Table) -> Self {
        Self {
            name: table.name().to_string(),
            columns: table.columns().iter().map(ColumnSchema::from_column).collect(),
            foreign_keys: table.foreign_keys().iter().map(ForeignKeySchema::from_foreign_key).collect(),
        }
    }
}

impl ColumnSchema {
    /// Convert a Column to a serializable ColumnSchema
    pub fn from_column(column: &Column) -> Self {
        let data_type = column_type_to_string(column.column_type());
        let typescript_type = column_type_to_typescript(column.column_type(), column.is_nullable());
        
        Self {
            name: column.name().to_string(),
            data_type,
            typescript_type,
            nullable: column.is_nullable(),
            primary_key: column.is_primary_key(),
            unique: column.is_unique(),
            auto_increment: column.is_auto_increment(),
            default_value: column.default_value().map(|s| s.to_string()),
        }
    }
}

impl ForeignKeySchema {
    /// Convert a ForeignKey to a serializable ForeignKeySchema
    pub fn from_foreign_key(fk: &ForeignKey) -> Self {
        Self {
            column: fk.column.clone(),
            references_table: fk.references_table.clone(),
            references_column: fk.references_column.clone(),
        }
    }
}

/// Convert ColumnType to a string representation
fn column_type_to_string(col_type: &ColumnType) -> String {
    match col_type {
        ColumnType::Integer => "integer".to_string(),
        ColumnType::BigInteger => "bigint".to_string(),
        ColumnType::Text => "text".to_string(),
        ColumnType::Varchar(len) => format!("varchar({})", len),
        ColumnType::Boolean => "boolean".to_string(),
        ColumnType::Float => "float".to_string(),
        ColumnType::Double => "double".to_string(),
        ColumnType::Decimal { precision, scale } => format!("decimal({},{})", precision, scale),
        ColumnType::Date => "date".to_string(),
        ColumnType::DateTime => "datetime".to_string(),
        ColumnType::Timestamp => "timestamp".to_string(),
        ColumnType::Json => "json".to_string(),
        ColumnType::Uuid => "uuid".to_string(),
        ColumnType::Binary => "binary".to_string(),
    }
}

/// Convert ColumnType to TypeScript type
fn column_type_to_typescript(col_type: &ColumnType, nullable: bool) -> String {
    let base_type = match col_type {
        ColumnType::Integer | ColumnType::BigInteger => "number",
        ColumnType::Float | ColumnType::Double => "number",
        ColumnType::Decimal { .. } => "number",
        ColumnType::Boolean => "boolean",
        ColumnType::Text | ColumnType::Varchar(_) => "string",
        ColumnType::Date | ColumnType::DateTime | ColumnType::Timestamp => "string",
        ColumnType::Uuid => "string",
        ColumnType::Json => "any",
        ColumnType::Binary => "Uint8Array",
    };
    
    if nullable {
        format!("{} | null", base_type)
    } else {
        base_type.to_string()
    }
}

/// Export schema from a list of tables
pub fn export_schema(tables: Vec<Table>) -> SchemaExport {
    SchemaExport {
        tables: tables.iter().map(TableSchema::from_table).collect(),
    }
}

/// Export schema as JSON string
pub fn export_schema_json(tables: Vec<Table>) -> Result<String, serde_json::Error> {
    let schema = export_schema(tables);
    serde_json::to_string_pretty(&schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Column, ColumnType};

    #[test]
    fn test_column_type_to_typescript() {
        assert_eq!(column_type_to_typescript(&ColumnType::Integer, false), "number");
        assert_eq!(column_type_to_typescript(&ColumnType::Integer, true), "number | null");
        assert_eq!(column_type_to_typescript(&ColumnType::Text, false), "string");
        assert_eq!(column_type_to_typescript(&ColumnType::Boolean, false), "boolean");
        assert_eq!(column_type_to_typescript(&ColumnType::Uuid, false), "string");
    }

    #[test]
    fn test_export_simple_table() {
        let mut table = Table::new("users");
        table.add_column(
            Column::new("id", ColumnType::Integer)
                .primary_key()
                .auto_increment()
        );
        table.add_column(
            Column::new("email", ColumnType::Varchar(255))
        );
        table.add_column(
            Column::new("age", ColumnType::Integer)
                .nullable(true)
        );

        let schema = export_schema(vec![table]);
        assert_eq!(schema.tables.len(), 1);
        assert_eq!(schema.tables[0].name, "users");
        assert_eq!(schema.tables[0].columns.len(), 3);
        
        let id_col = &schema.tables[0].columns[0];
        assert_eq!(id_col.name, "id");
        assert_eq!(id_col.typescript_type, "number");
        assert!(id_col.primary_key);
        
        let age_col = &schema.tables[0].columns[2];
        assert_eq!(age_col.name, "age");
        assert_eq!(age_col.typescript_type, "number | null");
        assert!(age_col.nullable);
    }
}