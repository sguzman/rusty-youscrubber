use log::{debug, info};

use sea_orm::EntityTrait;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm::{Schema, Statement};

use crate::constructors as ctor;
use crate::sea_orm_models as sea;

use crate::data;

async fn db_connect() -> DatabaseConnection {
    info!("Connecting to database");
    let mut opt = ConnectOptions::new("sqlite://data.db?mode=rwc");
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    debug!("{:#?}", opt);

    let db = Database::connect(opt).await.unwrap();
    info!("Connected to database");
    create_table::<sea::payload::Entity>(&db).await;
    create_table::<sea::automaticcaptions::Entity>(&db).await;
    create_table::<sea::caption::Entity>(&db).await;
    create_table::<sea::channelcategory::Entity>(&db).await;
    create_table::<sea::channeltag::Entity>(&db).await;
    create_table::<sea::channelthumbnail::Entity>(&db).await;
    create_table::<sea::chapter::Entity>(&db).await;
    create_table::<sea::entry::Entity>(&db).await;
    create_table::<sea::format::Entity>(&db).await;
    create_table::<sea::formatsortfield::Entity>(&db).await;
    create_table::<sea::fragment::Entity>(&db).await;
    create_table::<sea::heatmap::Entity>(&db).await;
    create_table::<sea::httpheader::Entity>(&db).await;
    create_table::<sea::requesteddownload::Entity>(&db).await;
    create_table::<sea::subtitle::Entity>(&db).await;
    create_table::<sea::subtitletype::Entity>(&db).await;
    create_table::<sea::version::Entity>(&db).await;
    create_table::<sea::videocategory::Entity>(&db).await;
    create_table::<sea::videothumbnail::Entity>(&db).await;

    db
}

async fn create_table<T>(db: &DatabaseConnection)
where
    T: EntityTrait,
{
    info!("Creating table");
    let db_sqlite = db.get_database_backend();
    let schema = Schema::new(db_sqlite);

    let b = db_sqlite.build(&schema.create_table_from_entity::<T>(T::default()));

    let result = db.execute(Statement::from(b)).await;

    match result {
        Ok(_) => info!("Table created"),
        Err(e) => {
            info!("Error: {}", e);
        }
    }
}

pub async fn insert(payload: data::Channel) {
    let db = db_connect().await;
    ctor::payload::create(&db, payload).await;
}
