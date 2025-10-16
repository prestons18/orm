use crate::error::Result;
use crate::query::{OrderDirection, QueryBuilder};
use crate::schema::Column;

#[derive(Debug, Clone, PartialEq)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

/// Enum wrapper for different query builder implementations
pub enum QueryBuilderEnum {
    SQLite(SQLiteQueryBuilder),
    MySQL(MySQLQueryBuilder),
}

impl QueryBuilder for QueryBuilderEnum {
    fn select(&mut self, columns: &[Column]) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.select(columns);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.select(columns);
            }
        }
        self
    }

    fn from(&mut self, table: &str) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.from(table);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.from(table);
            }
        }
        self
    }

    fn where_clause(&mut self, condition: &str) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.where_clause(condition);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.where_clause(condition);
            }
        }
        self
    }

    fn order_by(&mut self, column: &str, direction: OrderDirection) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.order_by(column, direction);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.order_by(column, direction);
            }
        }
        self
    }

    fn limit(&mut self, limit: u64) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.limit(limit);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.limit(limit);
            }
        }
        self
    }

    fn offset(&mut self, offset: u64) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.offset(offset);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.offset(offset);
            }
        }
        self
    }

    fn insert_into(&mut self, table: &str, columns: &[&str]) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.insert_into(table, columns);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.insert_into(table, columns);
            }
        }
        self
    }

    fn values(&mut self, values: &[&str]) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.values(values);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.values(values);
            }
        }
        self
    }

    fn update(&mut self, table: &str) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.update(table);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.update(table);
            }
        }
        self
    }

    fn set(&mut self, column: &str, value: &str) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.set(column, value);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.set(column, value);
            }
        }
        self
    }

    fn delete_from(&mut self, table: &str) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.delete_from(table);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.delete_from(table);
            }
        }
        self
    }

    fn returning(&mut self, columns: &[&str]) -> &mut Self {
        match self {
            QueryBuilderEnum::SQLite(builder) => {
                builder.returning(columns);
            }
            QueryBuilderEnum::MySQL(builder) => {
                builder.returning(columns);
            }
        }
        self
    }

    fn build(&self) -> Result<String> {
        match self {
            QueryBuilderEnum::SQLite(builder) => builder.build(),
            QueryBuilderEnum::MySQL(builder) => builder.build(),
        }
    }

    fn reset(&mut self) {
        match self {
            QueryBuilderEnum::SQLite(builder) => builder.reset(),
            QueryBuilderEnum::MySQL(builder) => builder.reset(),
        }
    }
}

pub struct SQLiteQueryBuilder {
    query_type: QueryType,
    columns: Vec<String>,
    table: Option<String>,
    where_clauses: Vec<String>,
    order_by: Vec<(String, OrderDirection)>,
    limit: Option<u64>,
    offset: Option<u64>,
    insert_table: Option<String>,
    insert_columns: Vec<String>,
    insert_values: Vec<Vec<String>>,
    update_table: Option<String>,
    update_sets: Vec<(String, String)>,
    delete_table: Option<String>,
    returning_columns: Vec<String>,
}

impl SQLiteQueryBuilder {
    pub fn new() -> Self {
        Self {
            query_type: QueryType::Select,
            columns: Vec::new(),
            table: None,
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            insert_table: None,
            insert_columns: Vec::new(),
            insert_values: Vec::new(),
            update_table: None,
            update_sets: Vec::new(),
            delete_table: None,
            returning_columns: Vec::new(),
        }
    }

    fn build_select(&self) -> Result<String> {
        let mut sql = String::from("SELECT ");

        if self.columns.is_empty() {
            sql.push('*');
        } else {
            sql.push_str(&self.columns.join(", "));
        }

        if let Some(table) = &self.table {
            sql.push_str(" FROM ");
            sql.push_str(table);
        }

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_clauses: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, dir)| format!("{} {}", col, dir))
                .collect();
            sql.push_str(&order_clauses.join(", "));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        Ok(sql)
    }

    fn build_insert(&self) -> Result<String> {
        let table = self.insert_table.as_ref().ok_or_else(|| {
            crate::error::Error::QueryError("No table specified for INSERT".to_string())
        })?;

        if self.insert_columns.is_empty() {
            return Err(crate::error::Error::QueryError(
                "No columns specified for INSERT".to_string(),
            ));
        }

        if self.insert_values.is_empty() {
            return Err(crate::error::Error::QueryError(
                "No values specified for INSERT".to_string(),
            ));
        }

        let mut sql = format!(
            "INSERT INTO {} ({}) VALUES ",
            table,
            self.insert_columns.join(", ")
        );

        let value_groups: Vec<String> = self
            .insert_values
            .iter()
            .map(|values| format!("({})", values.join(", ")))
            .collect();

        sql.push_str(&value_groups.join(", "));

        if !self.returning_columns.is_empty() {
            sql.push_str(" RETURNING ");
            sql.push_str(&self.returning_columns.join(", "));
        }

        Ok(sql)
    }

    fn build_update(&self) -> Result<String> {
        let table = self.update_table.as_ref().ok_or_else(|| {
            crate::error::Error::QueryError("No table specified for UPDATE".to_string())
        })?;

        if self.update_sets.is_empty() {
            return Err(crate::error::Error::QueryError(
                "No SET clauses specified for UPDATE".to_string(),
            ));
        }

        let mut sql = format!("UPDATE {} SET ", table);

        let set_clauses: Vec<String> = self
            .update_sets
            .iter()
            .map(|(col, val)| format!("{} = {}", col, val))
            .collect();

        sql.push_str(&set_clauses.join(", "));

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        if !self.returning_columns.is_empty() {
            sql.push_str(" RETURNING ");
            sql.push_str(&self.returning_columns.join(", "));
        }

        Ok(sql)
    }

    fn build_delete(&self) -> Result<String> {
        let table = self.delete_table.as_ref().ok_or_else(|| {
            crate::error::Error::QueryError("No table specified for DELETE".to_string())
        })?;

        let mut sql = format!("DELETE FROM {}", table);

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        if !self.returning_columns.is_empty() {
            sql.push_str(" RETURNING ");
            sql.push_str(&self.returning_columns.join(", "));
        }

        Ok(sql)
    }
}

impl QueryBuilder for SQLiteQueryBuilder {
    fn select(&mut self, columns: &[Column]) -> &mut Self {
        self.query_type = QueryType::Select;
        self.columns = columns.iter().map(|c| c.name().to_string()).collect();
        self
    }

    fn from(&mut self, table: &str) -> &mut Self {
        self.table = Some(table.to_string());
        self
    }

    fn where_clause(&mut self, condition: &str) -> &mut Self {
        self.where_clauses.push(condition.to_string());
        self
    }

    fn order_by(&mut self, column: &str, direction: OrderDirection) -> &mut Self {
        self.order_by.push((column.to_string(), direction));
        self
    }

    fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    fn insert_into(&mut self, table: &str, columns: &[&str]) -> &mut Self {
        self.query_type = QueryType::Insert;
        self.insert_table = Some(table.to_string());
        self.insert_columns = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    fn values(&mut self, values: &[&str]) -> &mut Self {
        let value_row = values.iter().map(|v| v.to_string()).collect();
        self.insert_values.push(value_row);
        self
    }

    fn update(&mut self, table: &str) -> &mut Self {
        self.query_type = QueryType::Update;
        self.update_table = Some(table.to_string());
        self
    }

    fn set(&mut self, column: &str, value: &str) -> &mut Self {
        self.update_sets.push((column.to_string(), value.to_string()));
        self
    }

    fn delete_from(&mut self, table: &str) -> &mut Self {
        self.query_type = QueryType::Delete;
        self.delete_table = Some(table.to_string());
        self
    }

    fn returning(&mut self, columns: &[&str]) -> &mut Self {
        self.returning_columns = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    fn build(&self) -> Result<String> {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        }
    }

    fn reset(&mut self) {
        self.query_type = QueryType::Select;
        self.columns.clear();
        self.table = None;
        self.where_clauses.clear();
        self.order_by.clear();
        self.limit = None;
        self.offset = None;
        self.insert_table = None;
        self.insert_columns.clear();
        self.insert_values.clear();
        self.update_table = None;
        self.update_sets.clear();
        self.delete_table = None;
        self.returning_columns.clear();
    }
}

pub struct MySQLQueryBuilder {
    query_type: QueryType,
    columns: Vec<String>,
    table: Option<String>,
    where_clauses: Vec<String>,
    order_by: Vec<(String, OrderDirection)>,
    limit: Option<u64>,
    offset: Option<u64>,
    insert_table: Option<String>,
    insert_columns: Vec<String>,
    insert_values: Vec<Vec<String>>,
    update_table: Option<String>,
    update_sets: Vec<(String, String)>,
    delete_table: Option<String>,
}

impl MySQLQueryBuilder {
    pub fn new() -> Self {
        Self {
            query_type: QueryType::Select,
            columns: Vec::new(),
            table: None,
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            insert_table: None,
            insert_columns: Vec::new(),
            insert_values: Vec::new(),
            update_table: None,
            update_sets: Vec::new(),
            delete_table: None,
        }
    }

    fn build_select(&self) -> Result<String> {
        let mut sql = String::from("SELECT ");

        if self.columns.is_empty() {
            sql.push('*');
        } else {
            sql.push_str(&self.columns.join(", "));
        }

        if let Some(table) = &self.table {
            sql.push_str(" FROM ");
            sql.push_str(table);
        }

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_clauses: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, dir)| format!("{} {}", col, dir))
                .collect();
            sql.push_str(&order_clauses.join(", "));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        Ok(sql)
    }

    fn build_insert(&self) -> Result<String> {
        let table = self.insert_table.as_ref().ok_or_else(|| {
            crate::error::Error::QueryError("No table specified for INSERT".to_string())
        })?;

        if self.insert_columns.is_empty() {
            return Err(crate::error::Error::QueryError(
                "No columns specified for INSERT".to_string(),
            ));
        }

        if self.insert_values.is_empty() {
            return Err(crate::error::Error::QueryError(
                "No values specified for INSERT".to_string(),
            ));
        }

        let mut sql = format!(
            "INSERT INTO {} ({}) VALUES ",
            table,
            self.insert_columns.join(", ")
        );

        let value_groups: Vec<String> = self
            .insert_values
            .iter()
            .map(|values| format!("({})", values.join(", ")))
            .collect();

        sql.push_str(&value_groups.join(", "));

        Ok(sql)
    }

    fn build_update(&self) -> Result<String> {
        let table = self.update_table.as_ref().ok_or_else(|| {
            crate::error::Error::QueryError("No table specified for UPDATE".to_string())
        })?;

        if self.update_sets.is_empty() {
            return Err(crate::error::Error::QueryError(
                "No SET clauses specified for UPDATE".to_string(),
            ));
        }

        let mut sql = format!("UPDATE {} SET ", table);

        let set_clauses: Vec<String> = self
            .update_sets
            .iter()
            .map(|(col, val)| format!("{} = {}", col, val))
            .collect();

        sql.push_str(&set_clauses.join(", "));

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        Ok(sql)
    }

    fn build_delete(&self) -> Result<String> {
        let table = self.delete_table.as_ref().ok_or_else(|| {
            crate::error::Error::QueryError("No table specified for DELETE".to_string())
        })?;

        let mut sql = format!("DELETE FROM {}", table);

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        Ok(sql)
    }
}

impl QueryBuilder for MySQLQueryBuilder {
    fn select(&mut self, columns: &[Column]) -> &mut Self {
        self.query_type = QueryType::Select;
        self.columns = columns.iter().map(|c| c.name().to_string()).collect();
        self
    }

    fn from(&mut self, table: &str) -> &mut Self {
        self.table = Some(table.to_string());
        self
    }

    fn where_clause(&mut self, condition: &str) -> &mut Self {
        self.where_clauses.push(condition.to_string());
        self
    }

    fn order_by(&mut self, column: &str, direction: OrderDirection) -> &mut Self {
        self.order_by.push((column.to_string(), direction));
        self
    }

    fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    fn insert_into(&mut self, table: &str, columns: &[&str]) -> &mut Self {
        self.query_type = QueryType::Insert;
        self.insert_table = Some(table.to_string());
        self.insert_columns = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    fn values(&mut self, values: &[&str]) -> &mut Self {
        let value_row = values.iter().map(|v| v.to_string()).collect();
        self.insert_values.push(value_row);
        self
    }

    fn update(&mut self, table: &str) -> &mut Self {
        self.query_type = QueryType::Update;
        self.update_table = Some(table.to_string());
        self
    }

    fn set(&mut self, column: &str, value: &str) -> &mut Self {
        self.update_sets.push((column.to_string(), value.to_string()));
        self
    }

    fn delete_from(&mut self, table: &str) -> &mut Self {
        self.query_type = QueryType::Delete;
        self.delete_table = Some(table.to_string());
        self
    }

    fn returning(&mut self, _columns: &[&str]) -> &mut Self {
        // MySQL doesn't support RETURNING, silently ignore
        self
    }

    fn build(&self) -> Result<String> {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        }
    }

    fn reset(&mut self) {
        self.query_type = QueryType::Select;
        self.columns.clear();
        self.table = None;
        self.where_clauses.clear();
        self.order_by.clear();
        self.limit = None;
        self.offset = None;
        self.insert_table = None;
        self.insert_columns.clear();
        self.insert_values.clear();
        self.update_table = None;
        self.update_sets.clear();
        self.delete_table = None;
    }
}