use crate::schema::Column;
use crate::query::builder::Dialect;

/// Represents a database table
#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    primary_key: Option<String>,
    indexes: Vec<Index>,
    foreign_keys: Vec<ForeignKey>,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub column: String,
    pub references_table: String,
    pub references_column: String,
    pub on_delete: Option<ForeignKeyAction>,
    pub on_update: Option<ForeignKeyAction>,
}

#[derive(Debug, Clone, Copy)]
pub enum ForeignKeyAction {
    Cascade,
    SetNull,
    Restrict,
    NoAction,
}

impl ForeignKeyAction {
    pub fn to_sql(&self) -> &str {
        match self {
            ForeignKeyAction::Cascade => "CASCADE",
            ForeignKeyAction::SetNull => "SET NULL",
            ForeignKeyAction::Restrict => "RESTRICT",
            ForeignKeyAction::NoAction => "NO ACTION",
        }
    }
}

impl Table {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            columns: Vec::new(),
            primary_key: None,
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
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

    pub fn add_index(&mut self, name: impl Into<String>, columns: Vec<String>, unique: bool) -> &mut Self {
        self.indexes.push(Index {
            name: name.into(),
            columns,
            unique,
        });
        self
    }

    pub fn add_foreign_key(&mut self, fk: ForeignKey) -> &mut Self {
        self.foreign_keys.push(fk);
        self
    }

    pub fn indexes(&self) -> &[Index] {
        &self.indexes
    }

    pub fn foreign_keys(&self) -> &[ForeignKey] {
        &self.foreign_keys
    }

    /// Generate CREATE TABLE SQL
    pub fn to_create_sql(&self, dialect: Dialect) -> String {
        let mut sql = format!("CREATE TABLE {} (\n", self.name);
        
        let column_defs: Vec<String> = self.columns
            .iter()
            .map(|col| format!("  {}", col.to_sql(dialect)))
            .collect();
        
        sql.push_str(&column_defs.join(",\n"));
        
        // Add foreign keys
        for fk in &self.foreign_keys {
            sql.push_str(",\n  ");
            sql.push_str(&format!(
                "FOREIGN KEY ({}) REFERENCES {}({})",
                fk.column, fk.references_table, fk.references_column
            ));
            
            if let Some(on_delete) = &fk.on_delete {
                sql.push_str(&format!(" ON DELETE {}", on_delete.to_sql()));
            }
            
            if let Some(on_update) = &fk.on_update {
                sql.push_str(&format!(" ON UPDATE {}", on_update.to_sql()));
            }
        }
        
        sql.push_str("\n)");
        sql
    }

    /// Generate DROP TABLE SQL
    pub fn to_drop_sql(&self) -> String {
        format!("DROP TABLE IF EXISTS {}", self.name)
    }
}