use mongodb::Database;

pub struct MongoPriceDB {
    pub db: Database,
}

pub struct MongoPriceDBConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub db_name: Option<String>,
}
