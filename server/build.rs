#[rustfmt::skip]
use {
    log::info,
    postgres::{Client, NoTls, Error as PostgresErr},
    refinery::{
        self,
        Migration,
    },
};

refinery::embed_migrations!("migrations");

fn main() -> Result<(), PostgresErr> {
    let mut conn = Client::connect("postgresql://postgres@localhost", NoTls)?;
    info!("Connected to database");

    Ok(())
}
