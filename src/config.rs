//! Config file definition and parsing logic

use std::collections::HashMap;

use serde::Deserialize;
use shrs::prelude::*;

#[derive(Debug, Deserialize)]
enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: Option<Theme>,
    pub environment: Option<HashMap<String, String>>
}

impl Config {
    pub fn apply(shell: &mut ShellConfig, config_file: &str) -> anyhow::Result<()> {
        let config: Config = toml::from_str(config_file)?;

        if let Some(theme) = config.theme {

        }

        if let Some(environment) = config.environment {
            for (k, v) in environment.iter() {
                // TODO maybe print warning message
                let _ = shell.env.set(k, v);
            }  
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
