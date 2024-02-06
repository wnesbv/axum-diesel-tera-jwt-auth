use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub img: String,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormLogin {
    pub email: String,
    pub password: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub exp: usize,
}