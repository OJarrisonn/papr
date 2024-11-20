use color_eyre::eyre::{Context, OptionExt};
use renderer::Renderer;
use serde::{Deserialize, Serialize};

pub mod renderer;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub renderer: Renderer,
}

impl Config {
    pub fn load() -> color_eyre::eyre::Result<Self> {
        let config_path = dirs::config_dir()
            .ok_or_eyre("Failed to get config directory")?
            .join("papr.toml");

        match std::fs::read_to_string(config_path) {
            Ok(config) => Ok(toml::from_str(&config).context("Failed to parse papr config")?),
            Err(_) => {
                let config = Self::default();

                config.save()?;

                Ok(config)
            }
        }
    }

    pub fn save(&self) -> color_eyre::eyre::Result<()> {
        let config_path = dirs::config_dir()
            .ok_or_eyre("Failed to get config directory")?
            .join("papr.toml");

        let config = toml::to_string(&self).context("Failed to serialize papr config")?;

        std::fs::write(config_path, config)?;

        Ok(())
    }
}
