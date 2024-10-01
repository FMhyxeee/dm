use axum::extract::State;
use axum::Json;

use crate::TestTable;

pub async fn index() -> &'static str {
    "Hello, world!"
}

pub async fn list_all(State(pool): State<sqlx::PgPool>) -> Json<Vec<TestTable>> {
    let lists = TestTable::list_all(&pool).await.unwrap();
    Json::from(lists)
}
