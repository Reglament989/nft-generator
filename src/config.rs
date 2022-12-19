use serde::Deserialize;

use crate::factory::factory::NFTFactory;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ipfs: String,
    pub description: String,
    pub external_url: Option<String>,
    pub name: String,
    pub edition: i32,
    pub factory: NFTFactory,
}
