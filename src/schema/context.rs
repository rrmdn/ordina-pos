use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use iron::prelude::*;
use std::env;

pub struct Context {
    pub pool: Pool<PostgresConnectionManager>,
}

impl juniper::Context for Context {}

pub fn context_factory(_: &mut Request) -> IronResult<Context> {
    let manager = PostgresConnectionManager::new(
        env::var("POSTGRES_CONNECTION_STRING").unwrap(),
        TlsMode::None,
    )
    .unwrap();
    Ok(Context {
        pool: Pool::new(manager).unwrap(),
    })
}

