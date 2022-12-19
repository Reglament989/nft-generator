use std::{collections::HashMap, env, path::PathBuf, time::Instant};

use crate::{
    config::Config,
    utils::{combine_pngs, hash},
};

use tokio::fs;

use super::{layer::Layer, nft::NFT};
use anyhow::Result;
use log::info;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct NFTFactory {
    count: i64,
    layers: Vec<Layer>,
    build_path: PathBuf, // "./build"
                         // options: Options,
}

// #[derive(Debug, Default, Deserialize, Clone)]
// pub struct Options {
//     conflicts: Option<HashMap<String, HashMap<String, Vec<String>>>>, // layer <source/trait name> - layer <conflict source> - Vec<Trait name>
//     forces: Option<HashMap<String, HashMap<String, Vec<String>>>>,
// }

impl NFTFactory {
    pub async fn clean(&mut self) -> Result<()> {
        let build_dir = env::current_dir()?.join(&self.build_path);
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).await?;
        }

        let path = build_dir.join("images/");
        fs::create_dir_all(path).await?;
        let path = build_dir.join("json/");
        fs::create_dir_all(path).await?;
        self.build_path = build_dir;
        Ok(())
    }

    pub async fn generate(&self, config: &Config, id: i64) -> Result<()> {
        info!("Looking up for nft");
        let start = Instant::now();
        let mut traits_of_layer = vec![];
        let mut all_conflicts: HashMap<String, Vec<String>> = HashMap::default(); // Layer Source - Vec<Conflicting traits>
        let mut all_forces: HashMap<String, Vec<String>> = HashMap::default(); // Layer Source - Vec<Conflicting traits>
        for layer in self.layers.clone() {
            match layer.choose(
                None,
                None,
                all_conflicts.get(layer.source_directory.as_str()),
                all_forces.get(layer.source_directory.as_str()),
            ) {
                Ok(lys) => {
                    for (source, conflict_traits) in lys.conflicts.clone() {
                        all_conflicts.insert(source, conflict_traits);
                    }
                    for (source, force_traits) in lys.forces.clone() {
                        all_forces.insert(source, force_traits);
                    }
                    traits_of_layer.push(lys)
                }
                Err(err) => {
                    dbg!(&err);
                    log::error!("{:#?}", err);
                }
            }
        }
        // log::info!("Picked traits: {:#?}", &traits_of_layer);
        let image_path = self.build_path.join(format!("images/{}.png", id));
        let tol = traits_of_layer.clone();
        let start_img = Instant::now();
        let img = combine_pngs(tol).await?;
        img.save(image_path)?;
        log::info!("Image processing: {:#?}", start_img.elapsed());
        let image_hash = hash(img.as_bytes());
        traits_of_layer.retain(|t| t.name != "!None");
        let external_url = {
            if let Some(url) = &config.external_url {
                Some(format!("{}/{}", url, id))
            } else {
                None
            }
        };
        let nft = NFT::new(
            format!("{} #{}", config.name, id),
            config.description.clone(),
            format!("ipfs://{}/{}.png", config.ipfs, id),
            image_hash,
            config.edition,
            traits_of_layer,
            external_url,
            None,
            None,
            None,
        );
        fs::write(
            self.build_path.join(format!("json/{}.json", id)),
            serde_json::to_string_pretty(&nft)?,
        )
        .await?;
        info!("NFT #{} {:#?}\nElapsed: {:#?}", id, nft, start.elapsed());
        Ok(())
    }

    pub async fn run(self, config: Config) -> Result<()> {
        // let mut limit = 0;
        for id in 0..self.count {
            self.generate(&config, id).await?;
            // limit += 1;
            // if limit == 50 {
            //     'inner: loop {
            //         let fut = futures.join_next().await;
            //         if fut.is_none() {
            //             limit = 0;
            //             break 'inner;
            //         }
            //     }
            // }
        }
        Ok(())
    }
}
