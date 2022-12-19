use serde::Serialize;

use crate::utils::timestamp;

use super::layer::LayerTrait;

#[derive(Debug, Serialize)]
pub struct NFT {
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    youtube_url: Option<String>,
    image: String, // anywhere, like this metadata anywhere
    #[serde(rename = "imageHash")]
    image_hash: String, // Hex kecak
    edition: i32,
    date: u128, // unix timestamp
    attributes: Vec<LayerTrait>,
}

impl NFT {
    pub fn new(
        name: String,
        description: String,
        image_url: String,
        image_hash: String,
        edition: i32,
        traits: Vec<LayerTrait>,
        external_url: Option<String>,
        background_color: Option<String>,
        animation_url: Option<String>,
        youtube_url: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            edition,
            image_hash,
            image: image_url,
            date: timestamp(),
            attributes: traits,
            external_url,
            background_color,
            animation_url,
            youtube_url,
        }
    }
}
