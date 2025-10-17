use orm::{prelude::*, query::QueryValue};

#[tokio::test]
async fn test_transaction_commit() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    backend.execute(r#"
        CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            balance INTEGER NOT NULL
        )
    "#, &[]).await?;

    // Insert initial data
    backend.execute("INSERT INTO accounts (name, balance) VALUES ('Alice', 100)", &[]).await?;
    backend.execute("INSERT INTO accounts (name, balance) VALUES ('Bob', 50)", &[]).await?;

    // Start a transaction
    let mut tx = db.begin_transaction().await?;

    // Perform operations within transaction
    tx.execute_params("UPDATE accounts SET balance = balance - ? WHERE name = ?", &[QueryValue::I64(30), QueryValue::String("Alice".to_string())]).await?;
    tx.execute_params("UPDATE accounts SET balance = balance + ? WHERE name = ?", &[QueryValue::I64(30), QueryValue::String("Bob".to_string())]).await?;

    // Commit the transaction
    tx.commit().await?;

    // Verify changes were committed
    let results = backend.fetch_all_params("SELECT name, balance FROM accounts ORDER BY name", &[]).await?;
    assert_eq!(results.len(), 2);
    
    let alice_balance = results[0].get("balance").and_then(|v| v.as_i64()).unwrap();
    let bob_balance = results[1].get("balance").and_then(|v| v.as_i64()).unwrap();
    
    assert_eq!(alice_balance, 70);
    assert_eq!(bob_balance, 80);

    Ok(())
}

#[tokio::test]
async fn test_transaction_rollback() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    backend.execute(r#"
        CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            balance INTEGER NOT NULL
        )
    "#, &[]).await?;

    // Insert initial data
    backend.execute("INSERT INTO accounts (name, balance) VALUES ('Alice', 100)", &[]).await?;
    backend.execute("INSERT INTO accounts (name, balance) VALUES ('Bob', 50)", &[]).await?;

    // Start a transaction
    let mut tx = db.begin_transaction().await?;

    // Perform operations within transaction
    tx.execute_params("UPDATE accounts SET balance = balance - ? WHERE name = ?", &[QueryValue::I64(30), QueryValue::String("Alice".to_string())]).await?;
    tx.execute_params("UPDATE accounts SET balance = balance + ? WHERE name = ?", &[QueryValue::I64(30), QueryValue::String("Bob".to_string())]).await?;

    // Rollback the transaction
    tx.rollback().await?;

    // Verify changes were rolled back
    let results = backend.fetch_all_params("SELECT name, balance FROM accounts ORDER BY name", &[]).await?;
    assert_eq!(results.len(), 2);
    
    let alice_balance = results[0].get("balance").and_then(|v| v.as_i64()).unwrap();
    let bob_balance = results[1].get("balance").and_then(|v| v.as_i64()).unwrap();
    
    // Balances should be unchanged
    assert_eq!(alice_balance, 100);
    assert_eq!(bob_balance, 50);

    Ok(())
}

#[tokio::test]
async fn test_transaction_auto_rollback_on_drop() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    backend.execute(r#"
        CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            balance INTEGER NOT NULL
        )
    "#, &[]).await?;

    // Insert initial data
    backend.execute("INSERT INTO accounts (name, balance) VALUES ('Alice', 100)", &[]).await?;

    {
        // Start a transaction in a scope
        let mut tx = db.begin_transaction().await?;
        tx.execute_params("UPDATE accounts SET balance = ? WHERE name = ?", &[QueryValue::I64(200), QueryValue::String("Alice".to_string())]).await?;
        // Transaction dropped without commit - should auto-rollback
    }

    // Verify changes were rolled back
    let results = backend.fetch_all_params("SELECT balance FROM accounts WHERE name = 'Alice'", &[]).await?;
    let alice_balance = results[0].get("balance").and_then(|v| v.as_i64()).unwrap();
    
    // Balance should be unchanged due to auto-rollback
    assert_eq!(alice_balance, 100);

    Ok(())
}

#[tokio::test]
async fn test_transaction_fetch_operations() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    backend.execute(r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )
    "#, &[]).await?;

    // Insert initial data
    backend.execute("INSERT INTO users (name, age) VALUES ('Alice', 30)", &[]).await?;
    backend.execute("INSERT INTO users (name, age) VALUES ('Bob', 25)", &[]).await?;

    // Start a transaction
    let mut tx = db.begin_transaction().await?;

    // Insert new user within transaction
    tx.execute_params("INSERT INTO users (name, age) VALUES (?, ?)", &[QueryValue::String("Charlie".to_string()), QueryValue::I32(35)]).await?;

    // Fetch all users within transaction (should see the new user)
    let users = tx.fetch_all_params("SELECT name, age FROM users ORDER BY name", &[]).await?;
    assert_eq!(users.len(), 3);

    // Fetch one user within transaction
    let alice = tx.fetch_one_params("SELECT name, age FROM users WHERE name = ?", &[QueryValue::String("Alice".to_string())]).await?;
    assert!(alice.is_some());
    let alice = alice.unwrap();
    assert_eq!(alice.get("name").and_then(|v| v.as_str()).unwrap(), "Alice");
    assert_eq!(alice.get("age").and_then(|v| v.as_i64()).unwrap(), 30);

    // Commit the transaction
    tx.commit().await?;

    // Verify the new user is visible outside the transaction
    let final_users = backend.fetch_all_params("SELECT name FROM users ORDER BY name", &[]).await?;
    assert_eq!(final_users.len(), 3);

    Ok(())
}

#[tokio::test]
async fn test_transaction_isolation() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;
    let backend = db.backend();

    // Create table
    backend.execute(r#"
        CREATE TABLE counters (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            value INTEGER NOT NULL
        )
    "#, &[]).await?;

    // Insert initial data
    backend.execute("INSERT INTO counters (value) VALUES (0)", &[]).await?;

    // Start a transaction
    let mut tx = db.begin_transaction().await?;

    // Update within transaction
    tx.execute_params("UPDATE counters SET value = ? WHERE id = ?", &[QueryValue::I64(100), QueryValue::I64(1)]).await?;

    // Read from outside the transaction (should see old value due to isolation)
    let outside_result = backend.fetch_one_params("SELECT value FROM counters WHERE id = ?", &[QueryValue::I64(1)]).await?;
    let outside_value = outside_result.unwrap().get("value").and_then(|v| v.as_i64()).unwrap();
    assert_eq!(outside_value, 0); // Should still be 0

    // Read from inside the transaction (should see new value)
    let inside_result = tx.fetch_one_params("SELECT value FROM counters WHERE id = ?", &[QueryValue::I64(1)]).await?;
    let inside_value = inside_result.unwrap().get("value").and_then(|v| v.as_i64()).unwrap();
    assert_eq!(inside_value, 100); // Should be updated to 100

    // Commit
    tx.commit().await?;

    // Now outside should see the committed value
    let final_result = backend.fetch_one_params("SELECT value FROM counters WHERE id = ?", &[QueryValue::I64(1)]).await?;
    let final_value = final_result.unwrap().get("value").and_then(|v| v.as_i64()).unwrap();
    assert_eq!(final_value, 100);

    Ok(())
}

#[tokio::test]
async fn test_transaction_error_handling() -> Result<()> {
    let db = Database::connect("sqlite::memory:").await?;

    // Create table
    db.backend().execute(r#"
        CREATE TABLE test (
            id INTEGER PRIMARY KEY,
            value TEXT NOT NULL
        )
    "#, &[]).await?;

    let mut tx = db.begin_transaction().await?;
    
    // This should succeed
    tx.execute_params("INSERT INTO test (id, value) VALUES (?, ?)", &[QueryValue::I64(1), QueryValue::String("test".to_string())]).await?;
    
    // This should fail (duplicate primary key)
    let result = tx.execute_params("INSERT INTO test (id, value) VALUES (?, ?)", &[QueryValue::I64(1), QueryValue::String("duplicate".to_string())]).await;
    assert!(result.is_err());

    // Transaction should still be usable after error
    tx.execute_params("INSERT INTO test (id, value) VALUES (?, ?)", &[QueryValue::I64(2), QueryValue::String("test2".to_string())]).await?;
    
    // Rollback to clean up
    tx.rollback().await?;

    Ok(())
}
