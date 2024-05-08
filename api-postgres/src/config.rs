use dotenv::dotenv;
use std::env::{self};
use tokio::sync::OnceCell;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
    max_pool: u32,
    min_pool: u32,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
}

impl Config {
    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }

    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn db_max_pool(&self) -> u32 {
        self.db.max_pool
    }

    pub fn db_min_pool(&self) -> u32 {
        self.db.min_pool
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("APP_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("APP_PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .unwrap(),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        max_pool: env::var("POSTGRES_MAX_POOL_SIZE")
            .unwrap_or_else(|_| String::from("10"))
            .parse::<u32>()
            .unwrap(),
        min_pool: env::var("POSTGRES_MIN_POOL_SIZE")
            .unwrap_or_else(|_| String::from("5"))
            .parse::<u32>()
            .unwrap(),
    };

    Config {
        server: server_config,
        db: database_config,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
