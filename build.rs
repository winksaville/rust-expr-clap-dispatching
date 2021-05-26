use vergen::{Config, vergen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen(Config::default())?;

    Ok(())
}