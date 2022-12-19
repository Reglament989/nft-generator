use std::fs;

use config::Config;
use log::info;
use tokio::time::Instant;

mod config;
mod factory;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let config: Config = serde_json::from_str(&fs::read_to_string(
        "/home/h/work/node/art-engine/nft-generator/config.json",
    )?)?;
    info!("Parsed config");
    let mut factory = config.factory.clone();
    factory.clean().await?;
    let start = Instant::now();
    factory.run(config).await?;
    log::info!("Success! Elapsed: {:#?}", start.elapsed());
    Ok(())
}
