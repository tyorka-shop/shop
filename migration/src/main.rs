use sea_orm_migration::prelude::*;
use url::{Position, Url};

#[async_std::main]
async fn main() {
    let cfg = config::load("tyorka-shop");

    let parsed = Url::parse(&cfg.database_uri).expect("Failed to parse database uri");

    let uri = format!("{}?mode=rwc", &parsed[..Position::AfterPath]);

    std::env::set_var("DATABASE_URL", &uri);

    cli::run_cli(migration::Migrator).await;
}
