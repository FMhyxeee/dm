mod api;

pub use api::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TestTable {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub salary: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
