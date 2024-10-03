use axum::extract::State;
use axum::Json;

use dm_common::{AppError, UserDetail, UserDetailCreate, UserDetailDelete, UserDetailUpdate};

pub async fn index() -> &'static str {
    "Hello, world!"
}

pub async fn list_all(State(pool): State<sqlx::PgPool>) -> Json<Vec<UserDetail>> {
    let lists = UserDetail::list_all(&pool).await.unwrap();
    Json::from(lists)
}

pub async fn create(
    State(pool): State<sqlx::PgPool>,
    Json(user_detail_create): Json<UserDetailCreate>,
) -> Result<Json<UserDetail>, AppError> {
    let new_record = UserDetail::create(
        &pool,
        user_detail_create.name,
        user_detail_create.age,
        user_detail_create.salary,
    )
    .await
    .unwrap();
    Ok(Json::from(new_record))
}

pub async fn update(
    State(pool): State<sqlx::PgPool>,
    Json(user_detail_update): Json<UserDetailUpdate>,
) -> Json<UserDetail> {
    let updated_record = UserDetail::update(
        &pool,
        user_detail_update.id,
        user_detail_update.name,
        user_detail_update.age,
        user_detail_update.salary,
    )
    .await
    .unwrap();
    Json::from(updated_record)
}

pub async fn delete_one(
    State(pool): State<sqlx::PgPool>,
    Json(user_detail_delete): Json<UserDetailDelete>,
) -> Json<UserDetail> {
    let deleted_record = UserDetail::delete_one(&pool, user_detail_delete.id)
        .await
        .unwrap();
    Json::from(deleted_record)
}
