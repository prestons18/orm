/// Represents a database column
#[derive(Debug, Clone)]
pub struct Column {
    name: String,
    column_type: ColumnType,
    nullable: bool,
    default: Option<String>,
    unique: bool,
    primary_key: bool,
    auto_increment: bool,
}

impl Column {
    pub fn new(name: impl Into<String>, column_type: ColumnType) -> Self {
        Self {
            name: name.into(),
            column_type,
            nullable: false,
            default: None,
            unique: false,
            primary_key: false,
            auto_increment: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn column_type(&self) -> &ColumnType {
        &self.column_type
    }

    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }

    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self
    }

    pub fn is_primary_key(&self) -> bool {
        self.primary_key
    }

    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }

    pub fn is_auto_increment(&self) -> bool {
        self.auto_increment
    }

    /// Generate SQL for this column definition
    pub fn to_sql(&self, dialect: crate::query::builder::Dialect) -> String {
        use crate::query::builder::Dialect;
        
        let mut sql = format!("{} {}", self.name, self.type_to_sql(dialect));
        
        if self.primary_key {
            sql.push_str(" PRIMARY KEY");
        }
        
        if self.auto_increment {
            match dialect {
                Dialect::SQLite => sql.push_str(" AUTOINCREMENT"),
                Dialect::MySQL => sql.push_str(" AUTO_INCREMENT"),
            }
        }
        
        if !self.nullable && !self.primary_key {
            sql.push_str(" NOT NULL");
        }
        
        if self.unique && !self.primary_key {
            sql.push_str(" UNIQUE");
        }
        
        if let Some(default) = &self.default {
            sql.push_str(&format!(" DEFAULT {}", default));
        }
        
        sql
    }

    fn type_to_sql(&self, dialect: crate::query::builder::Dialect) -> String {
        use crate::query::builder::Dialect;
        
        match (&self.column_type, dialect) {
            (ColumnType::Integer, Dialect::SQLite) => "INTEGER".to_string(),
            (ColumnType::Integer, Dialect::MySQL) => "INT".to_string(),
            // SQLite uses INTEGER for primary keys with AUTOINCREMENT
            (ColumnType::BigInteger, Dialect::SQLite) if self.auto_increment => "INTEGER".to_string(),
            (ColumnType::BigInteger, _) => "BIGINT".to_string(),
            (ColumnType::Text, _) => "TEXT".to_string(),
            (ColumnType::Varchar(len), _) => format!("VARCHAR({})", len),
            (ColumnType::Boolean, Dialect::SQLite) => "INTEGER".to_string(),
            (ColumnType::Boolean, Dialect::MySQL) => "BOOLEAN".to_string(),
            (ColumnType::Float, _) => "FLOAT".to_string(),
            (ColumnType::Double, _) => "DOUBLE".to_string(),
            (ColumnType::Decimal { precision, scale }, _) => {
                format!("DECIMAL({}, {})", precision, scale)
            }
            (ColumnType::Date, _) => "DATE".to_string(),
            (ColumnType::DateTime, Dialect::SQLite) => "TEXT".to_string(),
            (ColumnType::DateTime, Dialect::MySQL) => "DATETIME".to_string(),
            (ColumnType::Timestamp, _) => "TIMESTAMP".to_string(),
            (ColumnType::Json, Dialect::SQLite) => "TEXT".to_string(),
            (ColumnType::Json, Dialect::MySQL) => "JSON".to_string(),
            (ColumnType::Uuid, Dialect::SQLite) => "TEXT".to_string(),
            (ColumnType::Uuid, Dialect::MySQL) => "CHAR(36)".to_string(),
            (ColumnType::Binary, _) => "BLOB".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    BigInteger,
    Text,
    Varchar(usize),
    Boolean,
    Float,
    Double,
    Decimal { precision: u8, scale: u8 },
    Date,
    DateTime,
    Timestamp,
    Json,
    Uuid,
    Binary,
}