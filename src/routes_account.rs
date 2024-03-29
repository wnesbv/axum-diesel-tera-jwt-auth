use std::sync::{Arc};

use axum::{
    routing::{get},
    Extension,
    Router,
};

use tera::{Tera};

use tower_http::services::ServeDir;

use crate::{auth, profile, import_export, photo, common::{Pool}};


pub fn build_routes(conn: Pool) -> Router {

    let mut home_tera = Tera::default();
    home_tera.add_raw_templates(vec![
        ("base.html", include_str!("../templates/base.html")),
        ("index", include_str!("../templates/index.html")),
    ])
    .unwrap();
    
    let mut user_tera = Tera::default();
    user_tera.add_raw_templates(vec![
        ("base.html", include_str!("../templates/base.html")),
        ("users", include_str!("../templates/users.html")),
        ("user", include_str!("../templates/user.html")),
        ("signup", include_str!("../templates/signup.html")),
        ("login", include_str!("../templates/login.html")),
        ("update", include_str!("../templates/update.html")),
        ("password_change", include_str!("../templates/password_change.html")),
        ("export_csv", include_str!("../templates/export_csv.html")),
        ("photo", include_str!("../templates/photo.html")),
    ])
    .unwrap();

    let assets_path = std::env::current_dir().unwrap();

    let home_routes = Router::new()
        .nest(
            "/",
            Router::new()
                .route(
                    "/",
                    get(profile::handlers::index)
                )
                .layer(Extension(Arc::new(home_tera)))
        )
        .nest(
            "/account",
            Router::new()
                .route("/users", get(profile::handlers::users))
                .route("/user/:name", get(profile::handlers::user))
                .route(
                    "/login", get(auth::handlers::get_login).post(auth::handlers::post_login)
                )
                .route(
                    "/signup", get(profile::accreditation::get_signup).post(profile::accreditation::post_signup)
                )
                .route(
                    "/update", get(profile::accreditation::get_update).post(profile::accreditation::post_update_user)
                )
                .route(
                    "/password-change", get(profile::accreditation::get_password_change).post(profile::accreditation::post_password_change)
                )
                .route(
                    "/import", get(import_export::handlers::import_users)
                )
                .route(
                    "/export", get(import_export::handlers::get_export_users).post(import_export::handlers::export_users)
                )
                .route(
                    "/photo", get(photo::handlers::get_photo_users).post(photo::handlers::photo_users)
                )
                .layer(Extension(Arc::new(user_tera)))

        );
    Router::new()
        .nest("/", home_routes.with_state(conn)).nest_service(
        "/assets", ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),)
}
