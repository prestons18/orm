use orm::prelude::*;
use orm::query::QueryValue;
use std::collections::HashMap;

/// Blog Post model
#[derive(Debug, Clone)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub published: bool,
    pub created_at: Option<String>,
}

impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_key_value(&self) -> Option<Value> {
        self.id.map(Value::I64)
    }

    fn to_values(&self) -> HashMap<String, Value> {
        let mut values = HashMap::new();
        if let Some(id) = self.id {
            values.insert("id".to_string(), Value::I64(id));
        }
        values.insert("title".to_string(), Value::String(self.title.clone()));
        values.insert("content".to_string(), Value::String(self.content.clone()));
        values.insert("author_id".to_string(), Value::I64(self.author_id));
        values.insert("published".to_string(), Value::Bool(self.published));
        if let Some(created_at) = &self.created_at {
            values.insert("created_at".to_string(), Value::String(created_at.clone()));
        }
        values
    }

    fn columns() -> Vec<&'static str> {
        vec!["title", "content", "author_id", "published", "created_at"]
    }
}

impl FromRow for Post {
    fn from_row(row: &orm::model::Row) -> Result<Self> {
        let id = match row.get("id") {
            Some(Value::I64(n)) => Some(*n),
            _ => None,
        };

        let title = match row.get("title") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(Error::SerializationError("Missing title".to_string())),
        };

        let content = match row.get("content") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(Error::SerializationError("Missing content".to_string())),
        };

        let author_id = match row.get("author_id") {
            Some(Value::I64(n)) => *n,
            _ => return Err(Error::SerializationError("Missing author_id".to_string())),
        };

        let published = match row.get("published") {
            Some(Value::Bool(b)) => *b,
            Some(Value::I64(n)) => *n != 0, // SQLite stores booleans as integers
            _ => false,
        };

        let created_at = match row.get("created_at") {
            Some(Value::String(s)) => Some(s.clone()),
            _ => None,
        };

        Ok(Post {
            id,
            title,
            content,
            author_id,
            published,
            created_at,
        })
    }
}

impl ModelCrud for Post {}

/// Author model
#[derive(Debug, Clone)]
pub struct Author {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
}

impl Model for Author {
    fn table_name() -> &'static str {
        "authors"
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_key_value(&self) -> Option<Value> {
        self.id.map(Value::I64)
    }

    fn to_values(&self) -> HashMap<String, Value> {
        let mut values = HashMap::new();
        if let Some(id) = self.id {
            values.insert("id".to_string(), Value::I64(id));
        }
        values.insert("name".to_string(), Value::String(self.name.clone()));
        values.insert("email".to_string(), Value::String(self.email.clone()));
        values
    }

    fn columns() -> Vec<&'static str> {
        vec!["name", "email"]
    }
}

impl FromRow for Author {
    fn from_row(row: &orm::model::Row) -> Result<Self> {
        let id = match row.get("id") {
            Some(Value::I64(n)) => Some(*n),
            _ => None,
        };

        let name = match row.get("name") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(Error::SerializationError("Missing name".to_string())),
        };

        let email = match row.get("email") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(Error::SerializationError("Missing email".to_string())),
        };

        Ok(Author { id, name, email })
    }
}

impl ModelCrud for Author {}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to database
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    println!("🚀 Blog Application Example");
    println!("============================\n");

    // Create tables
    println!("📋 Creating tables...");
    create_tables(backend).await?;
    println!("✓ Tables created\n");

    // Create authors
    println!("👤 Creating authors...");
    let author1 = Author {
        id: None,
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
    };
    let author1 = Author::create(backend, &author1).await?;
    println!("✓ Created author: {} (ID: {})", author1.name, author1.id.unwrap());

    let author2 = Author {
        id: None,
        name: "Bob Smith".to_string(),
        email: "bob@example.com".to_string(),
    };
    let author2 = Author::create(backend, &author2).await?;
    println!("✓ Created author: {} (ID: {})\n", author2.name, author2.id.unwrap());

    // Create posts
    println!("📝 Creating blog posts...");
    let post1 = Post {
        id: None,
        title: "Getting Started with Rust".to_string(),
        content: "Rust is a systems programming language...".to_string(),
        author_id: author1.id.unwrap(),
        published: true,
        created_at: None,
    };
    let post1 = Post::create(backend, &post1).await?;
    println!("✓ Created post: \"{}\" by {}", post1.title, author1.name);

    let post2 = Post {
        id: None,
        title: "Advanced ORM Patterns".to_string(),
        content: "Object-Relational Mapping in Rust...".to_string(),
        author_id: author2.id.unwrap(),
        published: false,
        created_at: None,
    };
    let post2 = Post::create(backend, &post2).await?;
    println!("✓ Created post: \"{}\" by {}\n", post2.title, author2.name);

    // Query all posts
    println!("📚 All posts:");
    let all_posts = Post::all(backend).await?;
    for post in &all_posts {
        let status = if post.published { "Published" } else { "Draft" };
        println!("  - [{}] {}", status, post.title);
    }
    println!();

    // Query published posts only
    println!("✅ Published posts:");
    let published_posts = Post::query(backend)
        .where_eq("published", QueryValue::Bool(true))
        .get()
        .await?;
    for post in &published_posts {
        println!("  - {}", post.title);
    }
    println!();

    // Update a post
    println!("✏️  Publishing draft post...");
    let mut post2 = post2;
    post2.published = true;
    post2.update(backend).await?;
    println!("✓ Post \"{}\" is now published\n", post2.title);

    // Count posts
    let count = Post::count(backend).await?;
    println!("📊 Total posts: {}\n", count);

    // Delete a post
    println!("🗑️  Deleting a post...");
    post1.delete(backend).await?;
    println!("✓ Post deleted\n");

    // Final count
    let final_count = Post::count(backend).await?;
    println!("📊 Remaining posts: {}", final_count);

    Ok(())
}

async fn create_tables(backend: &dyn Backend) -> Result<()> {
    // Create authors table
    backend.execute_raw(
        "CREATE TABLE authors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )"
    ).await?;

    // Create posts table
    backend.execute_raw(
        "CREATE TABLE posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            author_id INTEGER NOT NULL,
            published INTEGER NOT NULL DEFAULT 0,
            created_at TEXT,
            FOREIGN KEY (author_id) REFERENCES authors(id)
        )"
    ).await?;

    Ok(())
}
