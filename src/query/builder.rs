use crate::error::Result;
use crate::query::{JoinType, OrderDirection, QueryBuilder, QueryValue};
use crate::schema::Column;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    SQLite,
    MySQL,
}

#[derive(Debug, Clone, PartialEq)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

/// Unified query builder for all database backends
pub struct QueryBuilderEnum {
    dialect: Dialect,
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
    joins: Vec<(JoinType, String, String)>, // (type, table, on_condition)
    group_by_columns: Vec<String>,
    having_clause: Option<String>,
    is_distinct: bool,
    params: Vec<QueryValue>,
}

impl QueryBuilderEnum {
    pub fn new(dialect: Dialect) -> Self {
        Self {
            dialect,
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
            joins: Vec::new(),
            group_by_columns: Vec::new(),
            having_clause: None,
            is_distinct: false,
            params: Vec::new(),
        }
    }

    /// Get the parameters for this query
    pub fn params(&self) -> &[QueryValue] {
        &self.params
    }

    /// Add a parameter and return its placeholder
    fn add_param(&mut self, value: QueryValue) -> String {
        self.params.push(value);
        match self.dialect {
            Dialect::SQLite => "?".to_string(),
            Dialect::MySQL => "?".to_string(),
        }
    }

    fn build_select(&self) -> Result<String> {
        let mut sql = String::from("SELECT ");

        if self.is_distinct {
            sql.push_str("DISTINCT ");
        }

        if self.columns.is_empty() {
            sql.push('*');
        } else {
            sql.push_str(&self.columns.join(", "));
        }

        if let Some(table) = &self.table {
            sql.push_str(" FROM ");
            sql.push_str(table);
        }

        // Add JOINs
        for (join_type, table, on) in &self.joins {
            let join_str = match join_type {
                JoinType::Inner => "INNER JOIN",
                JoinType::Left => "LEFT JOIN",
                JoinType::Right => "RIGHT JOIN",
                JoinType::Full => "FULL OUTER JOIN",
            };
            sql.push_str(&format!(" {} {} ON {}", join_str, table, on));
        }

        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }

        // Add GROUP BY
        if !self.group_by_columns.is_empty() {
            sql.push_str(" GROUP BY ");
            sql.push_str(&self.group_by_columns.join(", "));
        }

        // Add HAVING
        if let Some(having) = &self.having_clause {
            sql.push_str(" HAVING ");
            sql.push_str(having);
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

        // RETURNING is SQLite-specific
        if self.dialect == Dialect::SQLite && !self.returning_columns.is_empty() {
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

        // RETURNING is SQLite-specific
        if self.dialect == Dialect::SQLite && !self.returning_columns.is_empty() {
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

        // RETURNING is SQLite-specific
        if self.dialect == Dialect::SQLite && !self.returning_columns.is_empty() {
            sql.push_str(" RETURNING ");
            sql.push_str(&self.returning_columns.join(", "));
        }

        Ok(sql)
    }
}

impl QueryBuilder for QueryBuilderEnum {
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

    fn where_eq(&mut self, column: &str, value: QueryValue) -> &mut Self {
        let placeholder = self.add_param(value);
        self.where_clauses.push(format!("{} = {}", column, placeholder));
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

    fn values_params(&mut self, values: &[QueryValue]) -> &mut Self {
        let value_row: Vec<String> = values.iter().map(|v| self.add_param(v.clone())).collect();
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

    fn set_param(&mut self, column: &str, value: QueryValue) -> &mut Self {
        let placeholder = self.add_param(value);
        self.update_sets.push((column.to_string(), placeholder));
        self
    }

    fn delete_from(&mut self, table: &str) -> &mut Self {
        self.query_type = QueryType::Delete;
        self.delete_table = Some(table.to_string());
        self
    }

    fn returning(&mut self, columns: &[&str]) -> &mut Self {
        // Only store if SQLite, silently ignore for MySQL
        if self.dialect == Dialect::SQLite {
            self.returning_columns = columns.iter().map(|c| c.to_string()).collect();
        }
        self
    }

    fn join(&mut self, table: &str, on: &str, join_type: JoinType) -> &mut Self {
        self.joins.push((join_type, table.to_string(), on.to_string()));
        self
    }

    fn group_by(&mut self, columns: &[&str]) -> &mut Self {
        self.group_by_columns = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    fn having(&mut self, condition: &str) -> &mut Self {
        self.having_clause = Some(condition.to_string());
        self
    }

    fn distinct(&mut self) -> &mut Self {
        self.is_distinct = true;
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

    fn params(&self) -> &[QueryValue] {
        &self.params
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
        self.joins.clear();
        self.group_by_columns.clear();
        self.having_clause = None;
        self.is_distinct = false;
        self.params.clear();
    }
}

// Type aliases for backward compatibility
pub type SQLiteQueryBuilder = QueryBuilderEnum;
pub type MySQLQueryBuilder = QueryBuilderEnum;