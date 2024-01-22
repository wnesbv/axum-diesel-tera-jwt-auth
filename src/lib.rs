pub mod models;
pub mod schema;
pub mod common;
pub mod routes_account;
pub mod auth {
    pub mod handlers;
    pub mod repository;
    pub mod views;
}
pub mod profile {
    pub mod accreditation;
    pub mod handlers;
    pub mod repository;
    pub mod views;
}

