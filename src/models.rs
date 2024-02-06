use chrono::{DateTime, Utc};
use diesel::prelude::*;


#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub img: String,
    pub author_id: i32,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sessions {
    pub session_token: Vec<u8>,
    pub user_id: Option<i32>
}


#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}