use log::info;
use sea_orm::Database;

use sea_orm::DbErr;

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite::memory:";
const DB_NAME: &str = "youtube";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

use yourust::validate_json_files;

// Set logging to debug
fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
}

fn main() {
    init_logger();
    info!("Hello, world!");
    validate_json_files();
    info!("Goodbye, world!");
}
