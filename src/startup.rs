//! Collection of startup messages

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Startup {
    /// The default starfish logo
    Default,
    /// Don't display anything
    None,
    /// Show a fetch that displays system information
    Fetch,
    /// Gamble your money away
    Gacha,
    /// Run your own script
    Custom,
}
