use diesel::{Connection, SqliteConnection};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use crate::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(conn: &mut SqliteConnection) -> Result<(), Error> {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::InitDbError),
    }
}

fn get_database_path() -> Result<std::path::PathBuf, Error> {
    Ok(crate::util::get_app_data_dir()?.join("db.sqlite"))
}

fn get_connection() -> Result<SqliteConnection, Error> {
    let path = get_database_path()?;
    let conn =
        SqliteConnection::establish(path.to_str().unwrap()).map_err(|e| Error::DbConnection(e))?;
    Ok(conn)
}

pub(crate) fn init_db() -> Result<(), Error> {
    // create the database file if it does not exist
    let db_path = get_database_path()?;
    std::fs::File::create(&db_path).map_err(|e| Error::CouldNotCreateFile(db_path, e))?;
    let mut conn = get_connection()?;
    run_migration(&mut conn)
}
