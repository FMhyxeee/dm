use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TestTable {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub salary: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TestTable {
    pub fn new(name: String, age: i32, salary: Decimal) -> Self {
        Self {
            id: 0,
            name,
            age,
            salary,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub async fn list_all(db: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let lists = sqlx::query_as(
            "SELECT id, name, age, salary, created_at, updated_at FROM test_table ORDER BY id ASC",
        )
        .fetch_all(db)
        .await?;

        Ok(lists)
    }
}
