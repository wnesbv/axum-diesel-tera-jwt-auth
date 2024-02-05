use axum::{
    extract::{Multipart},
    response::{IntoResponse, Redirect, Html},
    Extension,
};

use std::io::prelude::*;
use std::fs::File;

use tera::{Context};

use crate::{
    common::{Templates},
};

pub use axum_macros::debug_handler;


#[debug_handler]
pub async fn get_photo_users(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    Html(templates.render("photo", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn photo_users(
    mut multipart: Multipart,
) -> impl IntoResponse {

    while let Some(field) = multipart.next_field().await.unwrap() {

        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let mut buffer = File::create(format!("./img/{}", file_name)).unwrap();
        buffer.write(&data).unwrap();

        println!("file_name .. {:?}", file_name);
    }

    return Redirect::to("/").into_response()
}