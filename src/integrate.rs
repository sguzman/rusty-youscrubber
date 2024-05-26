use log::{debug, warn};

use sea_orm::EntityTrait;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm::{Schema, Statement};

use crate::constructors as ctor;
use crate::sea_orm_models as sea;

use crate::data;

async fn db_connect() -> DatabaseConnection {
    debug!("Connecting to database");
    let mut opt = ConnectOptions::new("sqlite://data.db?mode=rwc");
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(false)
        //.sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("data"); // Setting default PostgreSQL schema

    debug!("{:#?}", opt);

    let db = Database::connect(opt).await.unwrap();
    debug!("Connected to database");
    create_tables(&db).await;

    db
}

async fn create_tables(db: &DatabaseConnection) {
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
    create_table::<sea::videotag::Entity>(&db).await;
}

async fn create_table<T>(db: &DatabaseConnection)
where
    T: EntityTrait,
{
    debug!("Creating table");
    let db_sqlite = db.get_database_backend();
    let schema = Schema::new(db_sqlite);

    let b = db_sqlite.build(&schema.create_table_from_entity::<T>(T::default()));

    let result = db.execute(Statement::from(b)).await;

    match result {
        Ok(_) => debug!("Table created"),
        Err(e) => {
            warn!("Error: {}", e);
        }
    }
}

pub async fn insert(payload: Vec<data::Channel>) {
    let db = db_connect().await;
    for pay in payload {
        ctor::payload::create(&db, pay).await;

        debug!("Record inserted");
    }

    db.close().await.unwrap();
}
