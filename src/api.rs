use axum::extract::State;
use axum::Json;

use crate::{TestTable, TestTableCreate, TestTableDelete, TestTableUpdate};

pub async fn index() -> &'static str {
    "Hello, world!"
}

pub async fn list_all(State(pool): State<sqlx::PgPool>) -> Json<Vec<TestTable>> {
    let lists = TestTable::list_all(&pool).await.unwrap();
    Json::from(lists)
}

pub async fn create(
    State(pool): State<sqlx::PgPool>,
    Json(test_table_create): Json<TestTableCreate>,
) -> Json<TestTable> {
    let new_record = TestTable::create(
        &pool,
        test_table_create.name,
        test_table_create.age,
        test_table_create.salary,
    )
    .await
    .unwrap();
    Json::from(new_record)
}

pub async fn update(
    State(pool): State<sqlx::PgPool>,
    Json(test_table_update): Json<TestTableUpdate>,
) -> Json<TestTable> {
    let updated_record = TestTable::update(
        &pool,
        test_table_update.id,
        test_table_update.name,
        test_table_update.age,
        test_table_update.salary,
    )
    .await
    .unwrap();
    Json::from(updated_record)
}

pub async fn delete_one(
    State(pool): State<sqlx::PgPool>,
    Json(test_table_delete): Json<TestTableDelete>,
) -> Json<TestTable> {
    let deleted_record = TestTable::delete_one(&pool, test_table_delete.id)
        .await
        .unwrap();
    Json::from(deleted_record)
}
