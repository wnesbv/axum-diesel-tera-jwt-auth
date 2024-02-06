
use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
    http::{
        Response, StatusCode,
    },
    body::Body,
    Extension,
};

use diesel::prelude::*;
use diesel_async::{RunQueryDsl};

use pbkdf2::{
    password_hash::{
        PasswordHash, PasswordVerifier
    },
    Pbkdf2
};

use jsonwebtoken::{encode, Header, EncodingKey};

use tera::{Context};

use crate::{
    common::{Pool, Templates},
    auth::models::{
        ListUser, FormLogin, Claims
    },
};
use crate::{schema};

pub use axum_macros::debug_handler;


pub async fn get_login(
    Extension(templates): Extension<Templates>
) -> impl IntoResponse {
    Html(templates.render("login", &Context::new()).unwrap())
}

#[debug_handler]
pub async fn post_login(
    State(pool): State<Pool>,
    Extension(templates): Extension<Templates>,
    Form(form): Form<FormLogin>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    let mut conn = pool.get().await.unwrap();
    use schema::users::dsl::*;

    let in_emails = users
        .filter(email.eq(form.email.clone()))
        .select(email)
        .first::<String>(&mut conn)
        .await
        .optional()
        .unwrap();

    let for_pass = users
        .filter(email.eq(form.email.clone()))
        .select(password)
        .first::<String>(&mut conn)
        .await
        .optional()
        .unwrap();

    let mut context = Context::new();

    if in_emails.is_none() {
        context.insert("for_email", "this email is not available..!");
        return Err(Html(templates.render("login", &context).unwrap()));
    }

    let pass = if let Some(for_pass) = for_pass {
        for_pass
    } else {
        "Error".to_string()
    };

    let parsed_hash = PasswordHash::new(&pass).unwrap();
    let veri = Pbkdf2.verify_password(
        form.password.clone().as_bytes(), &parsed_hash
    ).is_ok();
    if !veri {
        context.insert("for_password", "password is not correct..!");
        return Err(Html(templates.render("login", &context).unwrap()));
    };

    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT SECRET must be set");

    let in_user = users
        .filter(email.eq(form.email.clone()))
        .select(ListUser::as_select())
        .first::<ListUser>(&mut conn)
        .await
        .optional()
        .unwrap();

    let user = match in_user {
        Some(user) => user,
        None =>  return Err(Html(templates.render("login", &context).unwrap())),
    };

    let claims = Claims {
        id: user.id,
        email: user.email,
        username: user.username,
        exp: 10000000000,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref())).unwrap();

    return Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/account/users")
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Path={}; HttpOnly={}; SameSite={}",
                "visit",
                token,
                "/",
                "true",
                "lax",
            ),
        )
        .body(Body::from("not found"))
        .unwrap())
}