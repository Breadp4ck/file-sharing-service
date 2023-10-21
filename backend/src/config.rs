use std::{env::var, sync::OnceLock};

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| Config {
        db_url: var("DATABASE_URL").expect("DATABASE_URL must be specified"),
    })
}
