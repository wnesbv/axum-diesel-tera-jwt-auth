use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::date_config::date_format;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CsvUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: String,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct ExCsvUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub img: String,
}