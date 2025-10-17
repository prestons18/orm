use orm::prelude::*;
use orm::migration::{Migration, MigrationRunner, Schema};
use orm::schema::{ForeignKey, ForeignKeyAction};
use orm::query::builder::Dialect;
use async_trait::async_trait;

/// Migration to create users table
struct CreateUsersTable;

#[async_trait]
impl Migration for CreateUsersTable {
    fn name(&self) -> &str {
        "create_users_table"
    }

    fn version(&self) -> i64 {
        20241016_000001
    }

    async fn up(&self, schema: &mut Schema) -> Result<()> {
        schema.create_table("users", |table| {
            table.id("id");
            table.string("username", 50);
            table.string("email", 100);
            table.string("password_hash", 255);
            table.boolean("is_active");
            table.timestamps();
            table.index("idx_users_email", vec!["email".to_string()], true);
        });
        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<()> {
        schema.drop_table("users");
        Ok(())
    }
}

/// Migration to create posts table
struct CreatePostsTable;

#[async_trait]
impl Migration for CreatePostsTable {
    fn name(&self) -> &str {
        "create_posts_table"
    }

    fn version(&self) -> i64 {
        20241016_000002
    }

    async fn up(&self, schema: &mut Schema) -> Result<()> {
        schema.create_table("posts", |table| {
            table.id("id");
            table.string("title", 200);
            table.text("content");
            table.big_integer("user_id");
            table.boolean("published");
            table.integer("view_count");
            table.timestamps();
            
            table.foreign_key(ForeignKey {
                column: "user_id".to_string(),
                references_table: "users".to_string(),
                references_column: "id".to_string(),
                on_delete: Some(ForeignKeyAction::Cascade),
                on_update: None,
            });
            
            table.index("idx_posts_user_id", vec!["user_id".to_string()], false);
            table.index("idx_posts_published", vec!["published".to_string()], false);
        });
        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<()> {
        schema.drop_table("posts");
        Ok(())
    }
}

/// Migration to add tags table
struct CreateTagsTable;

#[async_trait]
impl Migration for CreateTagsTable {
    fn name(&self) -> &str {
        "create_tags_table"
    }

    fn version(&self) -> i64 {
        20241016_000003
    }

    async fn up(&self, schema: &mut Schema) -> Result<()> {
        schema.create_table("tags", |table| {
            table.id("id");
            table.string("name", 50);
            table.string("slug", 50);
            table.timestamps();
            table.index("idx_tags_slug", vec!["slug".to_string()], true);
        });

        // Create junction table for many-to-many relationship
        schema.create_table("post_tags", |table| {
            table.big_integer("post_id");
            table.big_integer("tag_id");
            
            table.foreign_key(ForeignKey {
                column: "post_id".to_string(),
                references_table: "posts".to_string(),
                references_column: "id".to_string(),
                on_delete: Some(ForeignKeyAction::Cascade),
                on_update: None,
            });
            
            table.foreign_key(ForeignKey {
                column: "tag_id".to_string(),
                references_table: "tags".to_string(),
                references_column: "id".to_string(),
                on_delete: Some(ForeignKeyAction::Cascade),
                on_update: None,
            });
            
            table.index("idx_post_tags", vec!["post_id".to_string(), "tag_id".to_string()], true);
        });
        
        Ok(())
    }

    async fn down(&self, schema: &mut Schema) -> Result<()> {
        schema.drop_table("post_tags");
        schema.drop_table("tags");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔧 Database Migration Example");
    println!("==============================\n");

    // Connect to database
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create migration runner
    let mut runner = MigrationRunner::new(backend, Dialect::SQLite);

    // Add migrations in order
    runner.add_migration(Box::new(CreateUsersTable));
    runner.add_migration(Box::new(CreatePostsTable));
    runner.add_migration(Box::new(CreateTagsTable));

    // Run migrations
    println!("📤 Running migrations...\n");
    runner.run_pending(backend).await?;
    
    println!("\n✅ All migrations completed successfully!");
    
    // Verify tables were created
    println!("\n📋 Verifying database schema...");
    let tables = backend.fetch_all_params(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
        &[]
    ).await?;
    
    println!("Created tables:");
    for table in tables {
        if let Some(name) = table.get("name").and_then(|v| {
            if let serde_json::Value::String(s) = v {
                Some(s)
            } else {
                None
            }
        }) {
            println!("  ✓ {}", name);
        }
    }

    // Example: Rollback last migration
    println!("\n🔄 Rolling back last migration...");
    runner.rollback(backend, 1).await?;
    
    println!("\n✅ Rollback completed!");

    Ok(())
}
