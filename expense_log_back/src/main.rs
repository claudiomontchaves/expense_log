pub mod error;
pub mod model;
pub mod repository;
pub mod routes;
pub mod service;

use crate::routes::routes::create_routes;
use app_properties::AppProperties;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let app_props = AppProperties::new();
    let server_address = format!("0.0.0.0:{}", app_props.get("server_port"));
    let max_connections = app_props.get("db_max_connections").parse::<u32>().unwrap();
    let db_uri = prepare_db_uri(app_props);

    let db_pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_uri)
        .await
        .expect("Unable to connect to Postgres DB.");

    let app = create_routes(db_pool);

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn prepare_db_uri(app_props: AppProperties) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        app_props.get("db_user"),
        app_props.get("db_password"),
        app_props.get("db_host"),
        app_props.get("db_port"),
        app_props.get("db_name")
    )
}
