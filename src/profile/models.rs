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

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormNewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub email: String,
    pub username: String,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormUpdateUser {
    pub email: String,
    pub username: String,
}


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct PasswordChange {
    pub password: String,
    #[serde(with = "date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPasswordChange {
    pub password: String,
}