use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CsvUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExCsvUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub updated_at: DateTime<Utc>,
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