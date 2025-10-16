use crate::backend::Backend;
use crate::error::Result;
use crate::query::builder::Dialect;
use crate::schema::{Column, ColumnType, ForeignKey, Table};
use async_trait::async_trait;

/// Represents a database migration
#[async_trait]
pub trait Migration: Send + Sync {
    /// Get the migration name
    fn name(&self) -> &str;

    /// Get the migration version (timestamp)
    fn version(&self) -> i64;

    /// Run the migration
    async fn up(&self, schema: &mut Schema) -> Result<()>;

    /// Rollback the migration
    async fn down(&self, schema: &mut Schema) -> Result<()>;
}

/// Schema builder for migrations
pub struct Schema {
    dialect: Dialect,
    operations: Vec<SchemaOperation>,
}

#[derive(Debug)]
enum SchemaOperation {
    CreateTable(Table),
    DropTable(String),
    AddColumn { table: String, column: Column },
    DropColumn { table: String, column: String },
    CreateIndex { table: String, name: String, columns: Vec<String>, unique: bool },
    DropIndex { name: String },
}

impl Schema {
    pub fn new(_backend: &dyn Backend, dialect: Dialect) -> Self {
        Self {
            dialect,
            operations: Vec::new(),
        }
    }

    /// Create a new table
    pub fn create_table<F>(&mut self, name: impl Into<String>, builder: F) -> &mut Self
    where
        F: FnOnce(&mut TableBuilder),
    {
        let mut table_builder = TableBuilder::new(name);
        builder(&mut table_builder);
        self.operations.push(SchemaOperation::CreateTable(table_builder.build()));
        self
    }

    /// Drop a table
    pub fn drop_table(&mut self, name: impl Into<String>) -> &mut Self {
        self.operations.push(SchemaOperation::DropTable(name.into()));
        self
    }

    /// Add a column to an existing table
    pub fn add_column(&mut self, table: impl Into<String>, column: Column) -> &mut Self {
        self.operations.push(SchemaOperation::AddColumn {
            table: table.into(),
            column,
        });
        self
    }

    /// Drop a column from a table
    pub fn drop_column(&mut self, table: impl Into<String>, column: impl Into<String>) -> &mut Self {
        self.operations.push(SchemaOperation::DropColumn {
            table: table.into(),
            column: column.into(),
        });
        self
    }

    /// Create an index
    pub fn create_index(
        &mut self,
        table: impl Into<String>,
        name: impl Into<String>,
        columns: Vec<String>,
        unique: bool,
    ) -> &mut Self {
        self.operations.push(SchemaOperation::CreateIndex {
            table: table.into(),
            name: name.into(),
            columns,
            unique,
        });
        self
    }

    /// Drop an index
    pub fn drop_index(&mut self, _table: impl Into<String>, name: impl Into<String>) -> &mut Self {
        self.operations.push(SchemaOperation::DropIndex {
            name: name.into(),
        });
        self
    }

    /// Execute all schema operations
    pub async fn execute(&self, backend: &dyn Backend) -> Result<()> {
        for operation in &self.operations {
            let sql = self.operation_to_sql(operation);
            backend.execute_raw(&sql).await?;
        }
        
        Ok(())
    }

    fn operation_to_sql(&self, operation: &SchemaOperation) -> String {
        match operation {
            SchemaOperation::CreateTable(table) => table.to_create_sql(self.dialect),
            SchemaOperation::DropTable(name) => format!("DROP TABLE IF EXISTS {}", name),
            SchemaOperation::AddColumn { table, column } => {
                format!("ALTER TABLE {} ADD COLUMN {}", table, column.to_sql(self.dialect))
            }
            SchemaOperation::DropColumn { table, column } => {
                format!("ALTER TABLE {} DROP COLUMN {}", table, column)
            }
            SchemaOperation::CreateIndex { table, name, columns, unique } => {
                let unique_str = if *unique { "UNIQUE " } else { "" };
                format!(
                    "CREATE {}INDEX {} ON {} ({})",
                    unique_str,
                    name,
                    table,
                    columns.join(", ")
                )
            }
            SchemaOperation::DropIndex { name } => {
                format!("DROP INDEX IF EXISTS {}", name)
            }
        }
    }
}

/// Table builder for creating tables in migrations
pub struct TableBuilder {
    table: Table,
}

impl TableBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            table: Table::new(name),
        }
    }

    /// Add an auto-incrementing ID column
    pub fn id(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::BigInteger)
            .primary_key()
            .auto_increment();
        self.table.add_column(column);
        self
    }

    /// Add a string column
    pub fn string(&mut self, name: impl Into<String>, length: usize) -> &mut Self {
        let column = Column::new(name, ColumnType::Varchar(length));
        self.table.add_column(column);
        self
    }

    /// Add a text column
    pub fn text(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Text);
        self.table.add_column(column);
        self
    }

    /// Add an integer column
    pub fn integer(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Integer);
        self.table.add_column(column);
        self
    }

    /// Add a big integer column
    pub fn big_integer(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::BigInteger);
        self.table.add_column(column);
        self
    }

    /// Add a boolean column
    pub fn boolean(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Boolean);
        self.table.add_column(column);
        self
    }

    /// Add a float column
    pub fn float(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Float);
        self.table.add_column(column);
        self
    }

    /// Add a double column
    pub fn double(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Double);
        self.table.add_column(column);
        self
    }

    /// Add a decimal column
    pub fn decimal(&mut self, name: impl Into<String>, precision: u8, scale: u8) -> &mut Self {
        let column = Column::new(name, ColumnType::Decimal { precision, scale });
        self.table.add_column(column);
        self
    }

    /// Add a date column
    pub fn date(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Date);
        self.table.add_column(column);
        self
    }

    /// Add a datetime column
    pub fn datetime(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::DateTime);
        self.table.add_column(column);
        self
    }

    /// Add a timestamp column
    pub fn timestamp(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Timestamp);
        self.table.add_column(column);
        self
    }

    /// Add a JSON column
    pub fn json(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Json);
        self.table.add_column(column);
        self
    }

    /// Add a UUID column
    pub fn uuid(&mut self, name: impl Into<String>) -> &mut Self {
        let column = Column::new(name, ColumnType::Uuid);
        self.table.add_column(column);
        self
    }

    /// Add timestamps (created_at, updated_at)
    pub fn timestamps(&mut self) -> &mut Self {
        self.timestamp("created_at");
        self.timestamp("updated_at");
        self
    }

    /// Add a foreign key
    pub fn foreign_key(&mut self, fk: ForeignKey) -> &mut Self {
        self.table.add_foreign_key(fk);
        self
    }

    /// Add an index
    pub fn index(&mut self, name: impl Into<String>, columns: Vec<String>, unique: bool) -> &mut Self {
        self.table.add_index(name, columns, unique);
        self
    }

    fn build(self) -> Table {
        self.table
    }
}

/// Migration runner
pub struct MigrationRunner {
    dialect: Dialect,
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationRunner {
    pub fn new(_backend: &dyn Backend, dialect: Dialect) -> Self {
        Self {
            dialect,
            migrations: Vec::new(),
        }
    }

    pub fn add_migration(&mut self, migration: Box<dyn Migration>) {
        self.migrations.push(migration);
    }

    /// Create migrations table if it doesn't exist
    async fn ensure_migrations_table(&self, backend: &dyn Backend) -> Result<()> {
        
        let sql = match self.dialect {
            Dialect::SQLite => {
                "CREATE TABLE IF NOT EXISTS migrations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    version BIGINT NOT NULL UNIQUE,
                    name TEXT NOT NULL,
                    executed_at TEXT NOT NULL
                )"
            }
            Dialect::MySQL => {
                "CREATE TABLE IF NOT EXISTS migrations (
                    id BIGINT PRIMARY KEY AUTO_INCREMENT,
                    version BIGINT NOT NULL UNIQUE,
                    name VARCHAR(255) NOT NULL,
                    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )"
            }
        };
        
        backend.execute_raw(sql).await?;
        Ok(())
    }

    /// Get executed migration versions
    async fn get_executed_versions(&self, backend: &dyn Backend) -> Result<Vec<i64>> {
        
        let rows = backend.fetch_all("SELECT version FROM migrations ORDER BY version").await?;
        
        let versions = rows
            .iter()
            .filter_map(|row| row.get("version").and_then(|v| v.as_i64()))
            .collect();
        
        Ok(versions)
    }

    /// Run all pending migrations
    pub async fn run_pending(&self, backend: &dyn Backend) -> Result<()> {
        self.ensure_migrations_table(backend).await?;
        let executed = self.get_executed_versions(backend).await?;
        
        for migration in &self.migrations {
            if !executed.contains(&migration.version()) {
                println!("Running migration: {} (v{})", migration.name(), migration.version());
                
                let mut schema = Schema::new(backend, self.dialect);
                migration.up(&mut schema).await?;
                schema.execute(backend).await?;
                
                // Record migration with parameterized query
                let sql = match self.dialect {
                    Dialect::SQLite => "INSERT INTO migrations (version, name, executed_at) VALUES (?, ?, datetime('now'))",
                    Dialect::MySQL => "INSERT INTO migrations (version, name, executed_at) VALUES (?, ?, NOW())",
                };
                let params = vec![
                    crate::query::QueryValue::I64(migration.version()),
                    crate::query::QueryValue::String(migration.name().to_string()),
                ];
                backend.execute(sql, &params).await?;
                
                println!("✓ Migration completed: {}", migration.name());
            }
        }
        
        Ok(())
    }

    /// Rollback the last N migrations
    pub async fn rollback(&self, backend: &dyn Backend, steps: usize) -> Result<()> {
        let executed = self.get_executed_versions(backend).await?;
        
        let to_rollback: Vec<_> = executed.iter().rev().take(steps).copied().collect();
        
        for version in to_rollback {
            if let Some(migration) = self.migrations.iter().find(|m| m.version() == version) {
                println!("Rolling back migration: {} (v{})", migration.name(), version);
                
                let mut schema = Schema::new(backend, self.dialect);
                migration.down(&mut schema).await?;
                schema.execute(backend).await?;
                
                // Remove migration record with parameterized query
                let sql = "DELETE FROM migrations WHERE version = ?";
                let params = vec![crate::query::QueryValue::I64(version)];
                backend.execute(sql, &params).await?;
                
                println!("✓ Rollback completed: {}", migration.name());
            }
        }
        
        Ok(())
    }
}