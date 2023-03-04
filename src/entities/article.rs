use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::article;

#[derive(Serialize)]
#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created: NaiveDateTime
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = article)]
pub struct CreateArticle {
    pub title: String,
    pub content: String
}
