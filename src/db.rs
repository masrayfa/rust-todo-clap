use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn establish_connection(database_uri: &str) {
    let mut opt = ConnectOptions::new(database_uri);
    opt.max_connections(100)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(8))
        .set_schema_search_path("my_schema");

    let database: Result<DatabaseConnection, DbErr> = Database::connect(opt).await;
}