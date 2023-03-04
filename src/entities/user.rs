use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use serde::{Serialize};
use crate::schema::app_user;

#[derive(Queryable)]
#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,

    #[serde(skip)]
    pub secret: String,

    pub created: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = app_user)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub secret: String
}
