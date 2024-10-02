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

#[derive(Debug, Serialize, Deserialize)]
pub struct TestTableCreate {
    pub name: String,
    pub age: i32,
    pub salary: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestTableUpdate {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub salary: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestTableDelete {
    pub id: i32,
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

    pub async fn create(
        db: &sqlx::PgPool,
        name: String,
        age: i32,
        salary: Decimal,
    ) -> Result<Self, sqlx::Error> {
        let new_record = sqlx::query_as(
            "INSERT INTO test_table (name, age, salary) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(age)
        .bind(salary)
        .fetch_one(db)
        .await?;

        Ok(new_record)
    }

    pub async fn update(
        db: &sqlx::PgPool,
        id: i32,
        name: String,
        age: i32,
        salary: Decimal,
    ) -> Result<Self, sqlx::Error> {
        let updated_record = sqlx::query_as(
            "UPDATE test_table SET name = $1, age = $2, salary = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $4 RETURNING *",
        ).bind(name)
        .bind(age)
        .bind(salary)
        .bind(id)
        .fetch_one(db)
        .await?;

        Ok(updated_record)
    }

    pub async fn delete_one(db: &sqlx::PgPool, id: i32) -> Result<Self, sqlx::Error> {
        let deleted_record = sqlx::query_as("DELETE FROM test_table WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(db)
            .await?;

        Ok(deleted_record)
    }
}
