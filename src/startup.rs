//! Collection of startup messages

use serde::Deserialize;
use shrs::{hooks::{HookFn, StartupCtx}, shell::{Shell, Runtime, Context}};

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

impl Startup {
    pub fn hook_fn(&self) -> HookFn<StartupCtx> {
        match self {
            Startup::Default => default_hook_fn,
            Startup::None => none_hook_fn,
            Startup::Fetch => todo!(),
            Startup::Gacha => todo!(),
            Startup::Custom => todo!(),
        }    
    }
}


fn default_hook_fn(_sh: &Shell, _sh_ctx: &mut Context, _sh_rt: &mut Runtime, _ctx: &StartupCtx) -> anyhow::Result<()> {
    let welcome_str = format!(r#"
      /\
   __/  \__    _______ _______ _______  ______ _______ _____ _______ _     _
  `.      .'   |______    |    |_____| |_____/ |______   |   |______ |_____|
    )    (     ______|    |    |     | |    \_ |       __|__ ______| |     |
   /__(\__\       Super Tasty Aesthetic Rusty Friendly Interactive SHell


                                          
"#);
    println!("{welcome_str}");
    Ok(())
}

fn none_hook_fn(_sh: &Shell, _sh_ctx: &mut Context, _sh_rt: &mut Runtime, _ctx: &StartupCtx) -> anyhow::Result<()> {
    Ok(())
}
