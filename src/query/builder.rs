use crate::error::Result;
use crate::query::{OrderDirection, QueryBuilder};
use crate::schema::Column;

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
    columns: Vec<String>,
    table: Option<String>,
    where_clauses: Vec<String>,
    order_by: Vec<(String, OrderDirection)>,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl SQLiteQueryBuilder {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            table: None,
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }
}

impl QueryBuilder for SQLiteQueryBuilder {
    fn select(&mut self, columns: &[Column]) -> &mut Self {
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

    fn build(&self) -> Result<String> {
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

    fn reset(&mut self) {
        self.columns.clear();
        self.table = None;
        self.where_clauses.clear();
        self.order_by.clear();
        self.limit = None;
        self.offset = None;
    }
}

pub struct MySQLQueryBuilder {
    columns: Vec<String>,
    table: Option<String>,
    where_clauses: Vec<String>,
    order_by: Vec<(String, OrderDirection)>,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl MySQLQueryBuilder {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            table: None,
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }
}

impl QueryBuilder for MySQLQueryBuilder {
    fn select(&mut self, columns: &[Column]) -> &mut Self {
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

    fn build(&self) -> Result<String> {
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

    fn reset(&mut self) {
        self.columns.clear();
        self.table = None;
        self.where_clauses.clear();
        self.order_by.clear();
        self.limit = None;
        self.offset = None;
    }
}