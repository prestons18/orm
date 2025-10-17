use orm::{prelude::*, query::QueryValue};
use std::collections::HashMap;

/// Test User model
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

impl ModelCrud for User {}

#[tokio::test]
async fn test_sqlite_crud_operations() -> Result<()> {
    // Connect to in-memory SQLite database
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    let create_table_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            age INTEGER NOT NULL
        )
    "#;
    backend.execute(create_table_sql, &[]).await?;

    // Test 1: Create a user with RETURNING
    let new_user = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };

    let created_user = User::create(backend, &new_user).await?;
    assert_eq!(created_user.name, "Alice");
    assert_eq!(created_user.email, "alice@example.com");
    assert_eq!(created_user.age, 30);
    assert!(created_user.id.is_some());

    // Test 2: Find by ID
    let found_user = User::find(backend, Value::I64(created_user.id.unwrap())).await?;
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.name, "Alice");

    // Test 3: Create more users
    let bob = User {
        id: None,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        age: 25,
    };
    User::create(backend, &bob).await?;

    let charlie = User {
        id: None,
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
        age: 35,
    };
    User::create(backend, &charlie).await?;

    // Test 4: Count all users
    let count = User::count(backend).await?;
    assert_eq!(count, 3);

    // Test 5: Find all users
    let all_users = User::all(backend).await?;
    assert_eq!(all_users.len(), 3);

    // Test 6: Query with WHERE clause
    let young_users = User::query(backend)
        .where_eq("age", QueryValue::I32(25))
        .get()
        .await?;
    assert_eq!(young_users.len(), 1);
    assert_eq!(young_users[0].name, "Bob");

    // Test 7: Query with ORDER BY
    let ordered_users = User::order_by(backend, "age", OrderDirection::Desc).await?;
    assert_eq!(ordered_users.len(), 3);
    assert_eq!(ordered_users[0].name, "Charlie"); // age 35
    assert_eq!(ordered_users[1].name, "Alice");   // age 30
    assert_eq!(ordered_users[2].name, "Bob");     // age 25

    // Test 8: Query with LIMIT
    let limited_users = User::take(backend, 2).await?;
    assert_eq!(limited_users.len(), 2);

    // Test 9: First user
    let first_user = User::first(backend).await?;
    assert!(first_user.is_some());

    // Test 10: Update a user
    let mut user_to_update = found_user.clone();
    user_to_update.age = 31;
    user_to_update.update(backend).await?;

    let updated_user = User::find(backend, Value::I64(user_to_update.id.unwrap())).await?;
    assert!(updated_user.is_some());
    assert_eq!(updated_user.unwrap().age, 31);

    // Test 11: Delete a user
    user_to_update.delete(backend).await?;
    let deleted_user = User::find(backend, Value::I64(user_to_update.id.unwrap())).await?;
    assert!(deleted_user.is_none());

    let remaining_count = User::count(backend).await?;
    assert_eq!(remaining_count, 1);

    Ok(())
}

#[tokio::test]
async fn test_query_builder() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    let create_table_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            age INTEGER NOT NULL
        )
    "#;
    backend.execute(create_table_sql, &[]).await?;

    // Insert test data
    for i in 1..=10 {
        let user = User {
            id: None,
            name: format!("User{}", i),
            email: format!("user{}@example.com", i),
            age: 20 + i,
        };
        User::create(backend, &user).await?;
    }

    // Test complex query
    let results = User::query(backend)
        .where_eq("age", QueryValue::I32(25))
        .order_by("age", OrderDirection::Asc)
        .limit(3)
        .get()
        .await?;

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].age, 25);
    assert_eq!(results[1].age, 26);
    assert_eq!(results[2].age, 27);

    // Test first with conditions
    let first_result = User::query(backend)
        .where_eq("age", QueryValue::I32(28))
        .order_by("age", OrderDirection::Asc)
        .first()
        .await?;

    assert!(first_result.is_some());
    assert_eq!(first_result.unwrap().age, 29);

    Ok(())
}
