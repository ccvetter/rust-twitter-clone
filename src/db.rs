use crate::error_handler::CustomError;
use diesel::migration::MigrationVersion;
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database URL not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get a db connection");
    run_migrations(&mut conn).expect("Failed to run migrations");
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get().map_err(|e| CustomError::DatabaseError(format!("Failed getting db connection: {}", e)))
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<Vec<MigrationVersion<'_>>, CustomError> {
    connection.run_pending_migrations(MIGRATIONS).map_err(|e| CustomError::DatabaseError(format!("Failed to run pending migrations: {}", e)))
}
