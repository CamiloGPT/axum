use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres};
use std::io;
use std::time::Duration;

use crate::utils::internal_error;

#[derive(Debug)]
pub enum PgError {
    Io(io::Error),
    Pg(sqlx::Error),
}

impl From<io::Error> for PgError {
    fn from(err: io::Error) -> Self {
        PgError::Io(err)
    }
}

impl From<sqlx::Error> for PgError {
    fn from(err: sqlx::Error) -> Self {
        PgError::Pg(err)
    }
}

pub async fn create_pool(database_url: &str, max_pool_size: u32, min_pool_size: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_pool_size)
        .min_connections(min_pool_size)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database")
}

pub struct DatabaseConnection(pub PoolConnection<Postgres>);

#[async_trait]
impl FromRequestParts<PgPool> for DatabaseConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        pool: &PgPool,
    ) -> Result<Self, Self::Rejection> {
        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}
