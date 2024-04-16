//! Config file definition and parsing logic

use std::collections::HashMap;

use serde::Deserialize;
use shrs::prelude::*;
use crate::theme::ColorTheme;
use crate::startup::Startup;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: Option<ColorTheme>,
    pub environment: Option<HashMap<String, String>>,
    pub alias: Option<HashMap<String, String>>,
    pub keybinding: Option<HashMap<String, String>>,
    pub startup: Option<Startup>,
}

impl Config {
    pub fn read(config_file: &str) -> anyhow::Result<Self> {
        let config_contents = std::fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&config_contents)?;
        Ok(config)
    }

    pub fn apply(self, shell: &mut ShellConfig) -> anyhow::Result<()> {

        if let Some(theme) = self.theme {
            shell.theme = theme.to_theme();
        }

        if let Some(environment) = self.environment {
            for (k, v) in environment.iter() {
                // TODO maybe print warning message
                let _ = shell.env.set(k, v);
            }  
        }

        if let Some(alias) = self.alias {
            for (k, v) in alias.iter() {
                let _ = shell.alias.set(k, AliasInfo::always(v));
            }  
        }

        if let Some(keybinding) = self.keybinding {
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::Config;
    use shrs::prelude::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let config_file = std::fs::read_to_string("example.toml")?;

        let mut myshell = ShellBuilder::default().build()?;
        Config::apply(&mut myshell, &config_file)?;

        Ok(())
    }
}
