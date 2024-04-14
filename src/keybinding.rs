//! Keybinding functions that can be used in config file

use std::process::Command;

use serde::Deserialize;

// TODO this method is not very extensible
#[derive(Debug, Deserialize)]
pub enum KeybindingFn {
    ClearScreen,
}

fn clear_screen() {
    let _ = Command::new("clear").spawn();
}

