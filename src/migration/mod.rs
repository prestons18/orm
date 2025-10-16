use crate::error::Result;
use crate::schema::Table;
use async_trait::async_trait;

/// Represents a database migration
#[async_trait]
pub trait Migration: Send + Sync {
    /// Get the migration name
    fn name(&self) -> &str;

    /// Get the migration version
    fn version(&self) -> i64;

    /// Run the migration
    async fn up(&self) -> Result<()>;

    /// Rollback the migration
    async fn down(&self) -> Result<()>;
}

/// Migration runner
pub struct MigrationRunner {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationRunner {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    pub fn add_migration(&mut self, migration: Box<dyn Migration>) {
        self.migrations.push(migration);
    }

    pub async fn run_pending(&self) -> Result<()> {
        // Run all pending migrations
        for migration in &self.migrations {
            migration.up().await?;
        }
        Ok(())
    }

    pub async fn rollback(&self, steps: usize) -> Result<()> {
        // Rollback specified number of migrations
        for migration in self.migrations.iter().rev().take(steps) {
            migration.down().await?;
        }
        Ok(())
    }
}

/// Builder for creating tables in migrations
pub struct CreateTable {
    table: Table,
}

impl CreateTable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            table: Table::new(name),
        }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
}