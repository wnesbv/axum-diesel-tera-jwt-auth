use tracing::info;
use axum::Router;

use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager
};

use axum_diesel::routes_account;


#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let account_router = routes_account::build_routes(pool.clone());

    let app = Router::new()
        .merge(account_router);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}


