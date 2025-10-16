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
    // Example usage (requires actual database connection)
    println!("User Model Example");
    println!("==================");
    println!();
    println!("Table: {}", User::table_name());
    println!("Primary Key: {}", User::primary_key());
    println!("Columns: {:?}", User::columns());
    println!();

    // Create a user instance
    let user = User {
        id: Some(1),
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };

    println!("User: {:?}", user);
    println!("Values: {:?}", user.to_values());

    Ok(())
}
