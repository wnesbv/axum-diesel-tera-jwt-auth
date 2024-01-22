use axum::{
    extract::{State},
};

use headers::Cookie;
use axum_extra::TypedHeader;

use jsonwebtoken::{
    errors,
    DecodingKey, TokenData
};

use crate::{
    common::{Pool},
    models::{
        ListUser, Claims
    },
};
use crate::{auth};

// pub use axum_macros::debug_handler;


pub async fn request_user(
    cookie: TypedHeader<Cookie>,
) -> Result<Option<Claims>, Option<errors::Error>> {

    let token = match cookie.get("visit") {
        Some(token) => token,
        None => return Ok(None),
    };

    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");

    let decode_token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    );
    match decode_token {
        Ok(claims) => return Ok(Some(claims.claims)),
        Err(err) => return Err(Some(err)),
    }
}


pub async fn request_token(
    TypedHeader(cookie): TypedHeader<Cookie>
) -> Result<TokenData<Claims>, Option<String>> {

    let token = match cookie.get("visit") {
        Some(token) => token,
        None => return Err(Some("Token not found".to_string())),
    };
    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");
    let decode_token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    );
    match decode_token {
        Ok(claims) => return Ok(claims),
        Err(err) => return Err(Some(err.to_string())),
    }
}


pub async fn request_auth(
    State(pool): State<Pool>,
    cookie: TypedHeader<Cookie>,
) -> Result<Option<ListUser>, Option<String>> {

    let claims = match request_token(cookie).await {
        Ok(claims) => claims,
        Err(_) => return Err(Some("token not found..!".to_string())),
    };

    let user = auth::repository::full_auth(State(pool), claims.claims.email).await;
    match user {
        Ok(user) => Ok(user),
        Err(_) => return Err(Some("User not found".to_string())),
    }
}


pub async fn user_token(
    cookie: TypedHeader<Cookie>,
) -> Result<Option<Claims>, &'static str> {

    let token = match cookie.get("visit") {
        Some(token) => token,
        None => return Ok(None),
    };

    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");

    let decode_token = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    ).unwrap();

    return Ok(Some(decode_token.claims))
}