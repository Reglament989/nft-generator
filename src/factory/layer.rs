use anyhow::Result;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use tokio::time::Instant;

fn default_percent_symbol() -> String {
    String::from("#")
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Layer {
    #[serde(rename = "source")]
    pub source_directory: String,
    pub trait_name: String,
    #[serde(default = "default_percent_symbol")]
    pub percent_symbol: String,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    pub conflicts: Option<HashMap<String, HashMap<String, Vec<String>>>>, // Trait value - Layer source - Vec<Trait value>
    pub forces: Option<HashMap<String, HashMap<String, Vec<String>>>>, // Trait value - Layer source - Vec<Trait value>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayerTrait {
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(rename = "trait_type")]
    pub name: String,
    pub value: String,
    #[serde(skip)]
    pub percent: f32,
    #[serde(skip)]
    pub nested_percent: Option<f32>,
    #[serde(skip)]
    pub conflicts: HashMap<String, Vec<String>>, // Layer source - Vec<Trait value>
    #[serde(skip)]
    pub forces: HashMap<String, Vec<String>>, // Layer source - Vec<Trait value>
}

impl LayerTrait {
    fn new(
        path: PathBuf,
        name: String,
        value: String,
        percent: f32,
        nested_percent: Option<f32>,
        conflicts: HashMap<String, Vec<String>>,
        forces: HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            path,
            name,
            value,
            percent,
            nested_percent,
            conflicts,
            forces,
        }
    }
}

impl Layer {
    pub fn choose(
        &self,
        path: Option<PathBuf>,
        nested_percent: Option<f32>,
        conflicts: Option<&Vec<String>>,
        forces: Option<&Vec<String>>,
    ) -> Result<LayerTrait> {
        let start = Instant::now();
        let paths = fs::read_dir(path.unwrap_or_else(|| {
            PathBuf::from("layers/").join(PathBuf::from(&self.source_directory))
        }))?;
        let mut all_files: Vec<LayerTrait> = vec![];
        let mut rng = rand::thread_rng();
        for path in paths {
            if path.is_err() {
                log::error!("{}", path.unwrap_err());
                continue;
            }
            let entry = path.unwrap();

            if entry.metadata()?.is_dir() {
                let name = entry.file_name();
                log::debug!("Scaning dir: {:?} Path: {:#?}", &name, entry.path());
                let rate = name
                    .to_str()
                    .ok_or(anyhow::anyhow!(
                        "Can't make from OsString &str, it's impossible to believe"
                    ))?
                    .rsplit_once(&self.percent_symbol)
                    .unwrap()
                    .1;
                log::debug!("{} is error {}", rate, rate.parse::<f32>().is_err());
                if rate.parse::<f32>()? > 0.0 {
                    all_files.push(self.choose(
                        Some(entry.path()),
                        Some(rate.parse::<f32>()?),
                        conflicts,
                        forces,
                    )?)
                }
            }

            if entry.metadata()?.is_file() {
                let name = entry.file_name();
                log::debug!("Scaning file: {:?}", &name);
                let (name, rate_with_ext) = name
                    .to_str()
                    .ok_or(anyhow::anyhow!(
                        "Can't make from OsString &str, it's impossible to believe"
                    ))?
                    .split_once(&self.percent_symbol)
                    .unwrap();
                let (rate, _) = rate_with_ext.rsplit_once(".").unwrap();
                let conflicts = {
                    if let Some(conflicts) = &self.conflicts {
                        if let Some(c) = conflicts.get(name) {
                            c.to_owned()
                        } else {
                            HashMap::default()
                        }
                    } else {
                        HashMap::default()
                    }
                };
                let forces = {
                    if let Some(forces) = &self.forces {
                        if let Some(c) = forces.get(name) {
                            c.to_owned()
                        } else {
                            HashMap::default()
                        }
                    } else {
                        HashMap::default()
                    }
                };
                all_files.push(LayerTrait::new(
                    entry.path(),
                    self.trait_name.to_owned(),
                    name.to_owned(),
                    rate.parse()?,
                    nested_percent,
                    conflicts,
                    forces,
                ))
            }
        }
        if let Some(forces) = forces {
            all_files.retain(|l| forces.contains(&l.value));
        }
        if let Some(conflicts) = conflicts {
            all_files.retain(|l| !conflicts.contains(&l.value));
        }
        let random_picked =
            all_files.choose_weighted(&mut rng, |a| a.nested_percent.unwrap_or(a.percent))?;
        let returned = all_files.remove(
            all_files
                .iter()
                .position(|value| value.value == random_picked.value)
                .unwrap(),
        );
        log::info!("Choose taken: {:#?}", start.elapsed());

        Ok(returned)
    }
}
