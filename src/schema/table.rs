use crate::schema::Column;

/// Represents a database table
#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    primary_key: Option<String>,
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
            primary_key: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_column(&mut self, column: Column) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn set_primary_key(&mut self, column: impl Into<String>) -> &mut Self {
        self.primary_key = Some(column.into());
        self
    }

    pub fn primary_key(&self) -> Option<&str> {
        self.primary_key.as_deref()
    }
}