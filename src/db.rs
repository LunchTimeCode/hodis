use std::env;
use std::sync::{Arc, Mutex};

use turso::{Builder, Connection};

pub enum Location {
    Memory,
    Local,
}

impl Location {
    pub fn from_env() -> Location {
        let in_memory = env::var("IN_MEMORY")
            .map(|s| s.parse::<bool>().unwrap_or_default())
            .unwrap_or(false);
        if in_memory {
            Location::Memory
        } else {
            Location::Local
        }
    }
}

pub struct Settings {
    location: Location,
}

impl Settings {
    pub fn from_env() -> Self {
        let in_mem = Location::from_env();
        Self { location: in_mem }
    }
}

pub type DB = Arc<Mutex<turso::Database>>;

pub async fn init(s: &Settings) -> DB {
    let db = match s.location {
        Location::Memory => Builder::new_local(":memory:").build().await.unwrap(),

        Location::Local => Builder::new_local("hodi.db").build().await.unwrap(),
    };

    let conn = db.connect().unwrap();
    migrate(&conn).await;

    Arc::new(Mutex::new(db))
}

async fn migrate(c: &Connection) {
    let init = include_str!("../migrations/init.sql");
    if let Err(e) = c.execute(init, ()).await {
        panic!("{e}")
    }
}

pub async fn get_connection(db: &DB) -> turso::Connection {
    db.as_ref().lock().unwrap().connect().unwrap()
}
