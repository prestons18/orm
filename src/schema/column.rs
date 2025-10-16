/// Represents a database column
#[derive(Debug, Clone)]
pub struct Column {
    name: String,
    column_type: ColumnType,
    nullable: bool,
    default: Option<String>,
}

impl Column {
    pub fn new(name: impl Into<String>, column_type: ColumnType) -> Self {
        Self {
            name: name.into(),
            column_type,
            nullable: false,
            default: None,
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