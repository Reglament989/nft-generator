use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_keccak::{Hasher, Sha3};

use image::DynamicImage;

use crate::factory::layer::LayerTrait;

pub async fn combine_pngs(mut traits: Vec<LayerTrait>) -> Result<DynamicImage> {
    let orgignal = image::open(traits.pop().unwrap().path)?;
    let mut img = orgignal.resize(350, 350, image::imageops::FilterType::Lanczos3);
    for layer in traits {
        log::info!("Process image.. {} value: {}", layer.name, layer.value);
        let orgignal = image::open(&layer.path)?;
        let top = orgignal.resize(350, 350, image::imageops::FilterType::Lanczos3);
        image::imageops::overlay(&mut img, &top, 0, 0);
    }
    Ok(img)
}

pub fn hash(bytes: &[u8]) -> String {
    let mut hasher = Sha3::v256();
    let mut output = [0u8; 32];
    hasher.update(bytes);
    hasher.finalize(&mut output);
    hex::encode(output)
}

pub fn timestamp() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}
