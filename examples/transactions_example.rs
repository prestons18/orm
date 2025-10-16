use orm::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Option<i64>,
    pub name: String,
    pub balance: i64,
}

impl Model for Account {
    fn table_name() -> &'static str {
        "accounts"
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
        values.insert("balance".to_string(), Value::I64(self.balance));
        values
    }

    fn columns() -> Vec<&'static str> {
        vec!["name", "balance"]
    }
}

impl FromRow for Account {
    fn from_row(row: &orm::model::Row) -> Result<Self> {
        let id = match row.get("id") {
            Some(Value::I64(n)) => Some(*n),
            _ => None,
        };

        let name = match row.get("name") {
            Some(Value::String(s)) => s.clone(),
            _ => return Err(Error::SerializationError("Missing name".to_string())),
        };

        let balance = match row.get("balance") {
            Some(Value::I64(n)) => *n,
            _ => return Err(Error::SerializationError("Missing balance".to_string())),
        };

        Ok(Account { id, name, balance })
    }
}

impl ModelCrud for Account {}

#[tokio::main]
async fn main() -> Result<()> {
    println!("💰 Transaction Example - Bank Transfer");
    println!("========================================\n");

    // Connect to database
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create accounts table
    backend.execute_raw(
        "CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            balance INTEGER NOT NULL
        )"
    ).await?;

    // Create initial accounts
    println!("📝 Creating accounts...");
    let alice = Account {
        id: None,
        name: "Alice".to_string(),
        balance: 1000,
    };
    let alice = Account::create(backend, &alice).await?;
    println!("  ✓ Alice's account: ${}", alice.balance);

    let bob = Account {
        id: None,
        name: "Bob".to_string(),
        balance: 500,
    };
    let bob = Account::create(backend, &bob).await?;
    println!("  ✓ Bob's account: ${}\n", bob.balance);

    // Successful transfer
    println!("💸 Transfer $200 from Alice to Bob...");
    match transfer(backend, alice.id.unwrap(), bob.id.unwrap(), 200).await {
        Ok(_) => {
            println!("  ✅ Transfer successful!\n");
            
            let alice_after = Account::find(backend, Value::I64(alice.id.unwrap())).await?.unwrap();
            let bob_after = Account::find(backend, Value::I64(bob.id.unwrap())).await?.unwrap();
            
            println!("  Final balances:");
            println!("    Alice: ${}", alice_after.balance);
            println!("    Bob: ${}\n", bob_after.balance);
        }
        Err(e) => println!("  ❌ Transfer failed: {}\n", e),
    }

    // Failed transfer (insufficient funds)
    println!("💸 Attempting to transfer $2000 from Alice to Bob...");
    match transfer(backend, alice.id.unwrap(), bob.id.unwrap(), 2000).await {
        Ok(_) => println!("  ✅ Transfer successful!\n"),
        Err(e) => {
            println!("  ❌ Transfer failed: {}\n", e);
            
            let alice_after = Account::find(backend, Value::I64(alice.id.unwrap())).await?.unwrap();
            let bob_after = Account::find(backend, Value::I64(bob.id.unwrap())).await?.unwrap();
            
            println!("  Balances unchanged (transaction rolled back):");
            println!("    Alice: ${}", alice_after.balance);
            println!("    Bob: ${}", bob_after.balance);
        }
    }

    Ok(())
}

/// Transfer money between accounts using a transaction
async fn transfer(backend: &dyn Backend, from_id: i64, to_id: i64, amount: i64) -> Result<()> {
    // Begin transaction
    let mut tx = backend.begin_transaction().await?;

    // Get source account
    let from_account_json = tx.fetch_one(
        &format!("SELECT * FROM accounts WHERE id = {}", from_id)
    ).await?;
    
    let from_account = match from_account_json {
        Some(json) => Account::from_json(&json)?,
        None => return Err(Error::QueryError("Source account not found".to_string())),
    };

    // Check sufficient funds
    if from_account.balance < amount {
        tx.rollback().await?;
        return Err(Error::QueryError("Insufficient funds".to_string()));
    }

    // Deduct from source account
    tx.execute(&format!(
        "UPDATE accounts SET balance = balance - {} WHERE id = {}",
        amount, from_id
    )).await?;

    // Add to destination account
    tx.execute(&format!(
        "UPDATE accounts SET balance = balance + {} WHERE id = {}",
        amount, to_id
    )).await?;

    // Commit transaction
    tx.commit().await?;

    Ok(())
}
