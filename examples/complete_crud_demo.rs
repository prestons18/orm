use orm::prelude::*;
use std::collections::HashMap;

/// Example User model
#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub age: i32,
}

impl Model for User {
    fn table_name() -> &'static str {
        "users"
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
        values.insert("age".to_string(), Value::I32(self.age));
        values
    }

    fn columns() -> Vec<&'static str> {
        vec!["name", "email", "age"]
    }
}

impl FromRow for User {
    fn from_row(row: &orm::model::Row) -> Result<Self> {
        let id = match row.get("id") {
            Some(Value::I64(n)) => Some(*n),
            Some(Value::I32(n)) => Some(*n as i64),
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

        let age = match row.get("age") {
            Some(Value::I32(n)) => *n,
            Some(Value::I64(n)) => *n as i32,
            _ => return Err(Error::SerializationError("Missing age".to_string())),
        };

        Ok(User {
            id,
            name,
            email,
            age,
        })
    }
}

// Implement CRUD operations
impl ModelCrud for User {}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== ORM Complete CRUD Demo ===\n");

    // Connect to in-memory SQLite database
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();
    println!("✓ Connected to SQLite database\n");

    // Create table
    let create_table_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            age INTEGER NOT NULL
        )
    "#;
    backend.execute_raw(create_table_sql).await?;
    println!("✓ Created 'users' table\n");

    // CREATE: Insert new users
    println!("--- CREATE Operations ---");
    let alice = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };
    let created_alice = User::create(backend, &alice).await?;
    println!("Created user: {:?}", created_alice);

    let bob = User {
        id: None,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        age: 25,
    };
    let created_bob = User::create(backend, &bob).await?;
    println!("Created user: {:?}", created_bob);

    let charlie = User {
        id: None,
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
        age: 35,
    };
    User::create(backend, &charlie).await?;
    println!("Created user: Charlie (age 35)\n");

    // READ: Find by ID
    println!("--- READ Operations ---");
    let found_user = User::find(backend, Value::I64(created_alice.id.unwrap())).await?;
    println!("Found by ID {}: {:?}", created_alice.id.unwrap(), found_user);

    // READ: Get all users
    let all_users = User::all(backend).await?;
    println!("\nAll users ({}): ", all_users.len());
    for user in &all_users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    // READ: Count users
    let count = User::count(backend).await?;
    println!("\nTotal user count: {}", count);

    // READ: Query with WHERE clause
    let young_users = User::where_clause(backend, "age < 30").await?;
    println!("\nUsers under 30:");
    for user in &young_users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    // READ: Query with ORDER BY
    let ordered_users = User::order_by(backend, "age", OrderDirection::Desc).await?;
    println!("\nUsers ordered by age (descending):");
    for user in &ordered_users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    // READ: Query with LIMIT
    let limited_users = User::take(backend, 2).await?;
    println!("\nFirst 2 users:");
    for user in &limited_users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    // READ: Complex query builder
    println!("\n--- Complex Query Builder ---");
    let complex_results = User::query(backend)
        .where_clause("age >= 25")
        .order_by("age", OrderDirection::Asc)
        .limit(2)
        .get()
        .await?;
    println!("Users age >= 25, ordered by age, limit 2:");
    for user in &complex_results {
        println!("  - {} (age: {})", user.name, user.age);
    }

    // UPDATE: Modify a user
    println!("\n--- UPDATE Operations ---");
    let mut user_to_update = found_user.unwrap();
    println!("Before update: {} is {} years old", user_to_update.name, user_to_update.age);
    user_to_update.age = 31;
    user_to_update.update(backend).await?;
    
    let updated_user = User::find(backend, Value::I64(user_to_update.id.unwrap())).await?;
    println!("After update: {} is {} years old", updated_user.as_ref().unwrap().name, updated_user.as_ref().unwrap().age);

    // DELETE: Remove a user
    println!("\n--- DELETE Operations ---");
    let count_before = User::count(backend).await?;
    println!("Users before delete: {}", count_before);
    
    user_to_update.delete(backend).await?;
    println!("Deleted user: {}", user_to_update.name);
    
    let count_after = User::count(backend).await?;
    println!("Users after delete: {}", count_after);

    // DELETE: Remove by condition
    let deleted_count = User::delete_where(backend, "age > 30").await?;
    println!("\nDeleted {} users with age > 30", deleted_count);
    
    let final_count = User::count(backend).await?;
    println!("Final user count: {}", final_count);

    // Show remaining users
    let remaining_users = User::all(backend).await?;
    println!("\nRemaining users:");
    for user in &remaining_users {
        println!("  - {} (age: {})", user.name, user.age);
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}
