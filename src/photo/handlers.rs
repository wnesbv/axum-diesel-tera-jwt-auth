use chrono::{Utc};
use axum::{
    extract::{State, Multipart},
    response::{IntoResponse, Redirect, Html},
    Extension,
};

use diesel::prelude::*;
use diesel_async::{RunQueryDsl};

use std::io::prelude::*;
use std::fs::File;

use tera::{Context};

use headers::Cookie;
use axum_extra::TypedHeader;

use crate::{
    common::{Pool, Templates},
    photo::models::{
        ImgUser
    },
};
use crate::{schema, auth};

pub use axum_macros::debug_handler;


#[debug_handler]
pub async fn get_photo_users(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    Html(templates.render("photo", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn photo_users(
    State(pool): State<Pool>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let token = auth::views::request_user(
        TypedHeader(cookie)
    ).await;
    let ok_token = match token {
        Ok(Some(expr)) => expr,
        Ok(None) => return Err(Redirect::to("/account/login").into_response()),
        Err(_) => return Err(Redirect::to("/account/login").into_response()),
    };

    let mut conn = pool.get().await.unwrap();
    use schema::users::dsl::*;

    while let Some(field) = multipart.next_field().await.unwrap() {

        let f_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let mut buffer = File::create(format!("./img/user/{}", f_name)).unwrap();
        buffer.write(&data).unwrap();

        let img_user = ImgUser {img: format!("/img/user/{}", f_name), updated_at: Utc::now()};

        let _ = diesel::update(users.filter(id.eq(ok_token.id)))
            .set(img_user)
            .execute(&mut conn)
            .await;

    }
    return Ok(Redirect::to("/").into_response())
}