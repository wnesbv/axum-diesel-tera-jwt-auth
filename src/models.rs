use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct ListUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Utc>,
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
    #[serde(with = "my_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct PasswordChange {
    pub password: String,
    #[serde(with = "my_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormUpdateUser {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormPasswordChange {
    pub password: String,
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




mod my_date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}