#[rustfmt::skip]
use {
    log::info,
    postgres::{Client, NoTls},
    refinery,
};

refinery::embed_migrations!("migrations");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Client::connect("postgresql://postgres:postgres@localhost", NoTls)?;
    info!("Connected to database");
    match migrations::runner().run(&mut conn) {
        Ok(_) => info!("Applied migrations"),
        Err(e) => {
            info!("Failed to apply migrations: {}", e);
            return Err(e.to_string().into());
        }
    };
    Ok(())
}
