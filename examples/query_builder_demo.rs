use orm::prelude::*;
use orm::query::builder::{Dialect, QueryBuilderEnum};
use orm::query::QueryValue;
use orm::schema::{Column, ColumnType};

fn main() -> Result<()> {
    println!("Enhanced Query Builder Demo");
    println!("============================\n");

    // Create a query builder
    let mut builder = QueryBuilderEnum::new(Dialect::SQLite);

    // Example 1: Simple SELECT with JOIN
    println!("1. SELECT with INNER JOIN:");
    let sql = builder
        .select(&[
            Column::new("users.id", ColumnType::BigInteger),
            Column::new("users.name", ColumnType::Text),
            Column::new("posts.title", ColumnType::Text),
        ])
        .from("users")
        .inner_join("posts", "posts.user_id = users.id")
        .where_eq("users.age", QueryValue::I32(18))
        .order_by("users.name", orm::query::OrderDirection::Asc)
        .build()?;
    println!("{}\n", sql);

    // Example 2: SELECT with GROUP BY and HAVING
    builder.reset();
    println!("2. SELECT with GROUP BY and HAVING:");
    let sql = builder
        .select(&[
            Column::new("department", ColumnType::Text),
            Column::new("COUNT(*) as employee_count", ColumnType::BigInteger),
            Column::new("AVG(salary) as avg_salary", ColumnType::Double),
        ])
        .from("employees")
        .group_by(&["department"])
        .having("COUNT(*) > 5")
        .order_by("avg_salary", orm::query::OrderDirection::Desc)
        .build()?;
    println!("{}\n", sql);

    // Example 3: DISTINCT SELECT
    builder.reset();
    println!("3. SELECT DISTINCT:");
    let sql = builder
        .distinct()
        .select(&[Column::new("country", ColumnType::Text)])
        .from("customers")
        .order_by("country", orm::query::OrderDirection::Asc)
        .build()?;
    println!("{}\n", sql);

    // Example 4: Multiple JOINs
    builder.reset();
    println!("4. Multiple JOINs:");
    let sql = builder
        .select(&[
            Column::new("orders.id", ColumnType::BigInteger),
            Column::new("customers.name", ColumnType::Text),
            Column::new("products.name", ColumnType::Text),
            Column::new("order_items.quantity", ColumnType::Integer),
        ])
        .from("orders")
        .inner_join("customers", "customers.id = orders.customer_id")
        .inner_join("order_items", "order_items.order_id = orders.id")
        .inner_join("products", "products.id = order_items.product_id")
        .where_eq("orders.status", QueryValue::String("completed".to_string()))
        .limit(10)
        .build()?;
    println!("{}\n", sql);

    // Example 5: LEFT JOIN with aggregation
    builder.reset();
    println!("5. LEFT JOIN with aggregation:");
    let sql = builder
        .select(&[
            Column::new("users.name", ColumnType::Text),
            Column::new("COUNT(posts.id) as post_count", ColumnType::BigInteger),
        ])
        .from("users")
        .left_join("posts", "posts.user_id = users.id")
        .group_by(&["users.id", "users.name"])
        .order_by("post_count", orm::query::OrderDirection::Desc)
        .limit(20)
        .build()?;
    println!("{}\n", sql);

    // Example 6: Complex query with all features
    builder.reset();
    println!("6. Complex query with all features:");
    let sql = builder
        .distinct()
        .select(&[
            Column::new("categories.name", ColumnType::Text),
            Column::new("COUNT(DISTINCT products.id) as product_count", ColumnType::BigInteger),
            Column::new("SUM(order_items.quantity) as total_sold", ColumnType::BigInteger),
        ])
        .from("categories")
        .inner_join("products", "products.category_id = categories.id")
        .left_join("order_items", "order_items.product_id = products.id")
        .where_eq("categories.active", QueryValue::Bool(true))
        .group_by(&["categories.id", "categories.name"])
        .having("COUNT(DISTINCT products.id) > 0")
        .order_by("total_sold", orm::query::OrderDirection::Desc)
        .limit(10)
        .offset(0)
        .build()?;
    println!("{}\n", sql);

    // Example 7: MySQL dialect (no RETURNING)
    let mut mysql_builder = QueryBuilderEnum::new(Dialect::MySQL);
    println!("7. MySQL INSERT (RETURNING ignored):");
    let sql = mysql_builder
        .insert_into("users", &["name", "email"])
        .values_params(&[
            QueryValue::String("John Doe".to_string()),
            QueryValue::String("john@example.com".to_string())
        ])
        .returning(&["id"]) // This will be ignored for MySQL
        .build()?;
    println!("{}\n", sql);

    // Example 8: SQLite with RETURNING
    let mut sqlite_builder = QueryBuilderEnum::new(Dialect::SQLite);
    println!("8. SQLite INSERT with RETURNING:");
    let sql = sqlite_builder
        .insert_into("users", &["name", "email"])
        .values_params(&[
            QueryValue::String("Jane Doe".to_string()),
            QueryValue::String("jane@example.com".to_string())
        ])
        .returning(&["id", "name"])
        .build()?;
    println!("{}\n", sql);

    Ok(())
}
