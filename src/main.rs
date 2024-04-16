mod keybinding;
mod prompt;
mod config;
mod theme;
mod startup;
mod gacha;

use std::{
    fs,
    io::{stdout, BufWriter},
    path::PathBuf,
    process::Command,
};

use config::Config;
use log::{LevelFilter, warn};
use prompt::SimplePrompt;
use shrs::crossterm::{Attribute, Color};
use shrs::{
    history::FileBackedHistory,
    keybindings,
    prelude::{cursor_buffer::CursorBuffer, styled_buf::StyledBuf, *},
};
use shrs_cd_stack::{CdStackPlugin, CdStackState};
use shrs_cd_tools::git;
use shrs_command_timer::{CommandTimerPlugin, CommandTimerState};
use shrs_rhai::RhaiPlugin;
use shrs_rhai_completion::CompletionsPlugin;
use simplelog::{TermLogger, TerminalMode, ColorChoice, CombinedLogger};

fn main() {

    // Setup logger
    let logger_config = simplelog::ConfigBuilder::new()
        .set_time_format(String::new())
        .build();
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, logger_config, TerminalMode::Mixed, ColorChoice::Auto)
    ]).expect("failed to start logger");

    // Initialize the directory we will be using to hold our configuration and metadata files
    // TODO modify this using env var
    let config_dir = dirs::home_dir().expect("Unable to get user home directory").as_path().join(".config/starfish");
    // also log when creating dir
    // TODO ignore errors for now (we dont care if dir already exists)
    let _ = fs::create_dir_all(config_dir.clone());

    // Load environment variables from calling shell
    let mut env = Env::default();
    let _ = env.load();
    let _ = env.set("SHELL_NAME", "starfish");

    let builtins = Builtins::default();

    // =-=-= Completion =-=-=
    // Get list of binaries in path and initialize the completer to autocomplete command names
    let path_string = env.get("PATH").unwrap().to_string();
    let mut completer = DefaultCompleter::default();
    completer.register(Rule::new(
        Pred::new(cmdname_pred),
        Box::new(cmdname_action(path_string)),
    ));
    completer.register(Rule::new(
        Pred::new(cmdname_pred),
        Box::new(builtin_cmdname_action(&builtins)),
    ));

    // =-=-= Menu =-=-=-=
    let menu = DefaultMenu::default();

    // =-=-= History =-=-=
    // Use history that writes to file on disk
    let history_file = config_dir.as_path().join("history");
    let history = FileBackedHistory::new(history_file).expect("Could not open history file");

    // =-=-= Keybindings =-=-=
    // Add basic keybindings
    let keybinding = keybindings! {
        |state|
        "C-l" => ("Clear the screen", { let _ = Command::new("clear").spawn(); }),
        "C-p" => ("Move up one in the command history", {
            if let Some(cd_state) = state.ctx.state.get_mut::<CdStackState>() {
                if let Some(new_path) = cd_state.down() {
                    set_working_dir(state.sh, state.ctx, state.rt, &new_path, false).unwrap();
                }
            }
        }),
        "C-n" => ("Move down one in the command history", {
            if let Some(cd_state) = state.ctx.state.get_mut::<CdStackState>() {
                if let Some(new_path) = cd_state.up() {
                    set_working_dir(state.sh, state.ctx, state.rt, &new_path, false).unwrap();
                }
            }
        }),
    };

    // =-=-= Prompt =-=-=
    let prompt = SimplePrompt;

    // =-=-= Readline =-=-=
    // Initialize readline with all of our components

    let readline = LineBuilder::default()
        .with_menu(menu)
        .with_prompt(prompt)
        .build()
        .expect("Could not construct readline");

    let alias = Alias::new();

    // =-=-= Hooks =-=-=
    // Create a hook that prints a welcome message on startup
    let startup_msg: HookFn<StartupCtx> = |_sh: &Shell,
                                           _sh_ctx: &mut Context,
                                           _sh_rt: &mut Runtime,
                                           _ctx: &StartupCtx|
     -> anyhow::Result<()> {
        let welcome_str = format!(r#"
      /\
   __/  \__    _______ _______ _______  ______ _______ _____ _______ _     _
  `.      .'   |______    |    |_____| |_____/ |______   |   |______ |_____|
    )    (     ______|    |    |     | |    \_ |       __|__ ______| |     |
   /__(\__\       Super Tasty Aesthetic Rusty Friendly Interactive SHell


                                          
        "#);
        println!("{welcome_str}");
        Ok(())
    };
    let mut hooks = Hooks::new();
    hooks.insert(startup_msg);

    // =-=-= Shell =-=-=
    // Construct the final shell
    let mut myshell = ShellBuilder::default()
        .with_completer(completer)
        .with_hooks(hooks)
        .with_env(env)
        .with_alias(alias)
        .with_readline(readline)
        .with_history(history)
        .with_keybinding(keybinding)
        .with_plugin(CommandTimerPlugin)
        .with_plugin(CdStackPlugin)
        .with_plugin(RhaiPlugin)
        .with_plugin(CompletionsPlugin)
        .build()
        .expect("Could not construct shell");

    // Read and apply config file
    let config_file = "example.toml";
    if let Ok(config) = Config::read(config_file) {
        config.apply(&mut myshell).unwrap();
    } else {
        // TODO prompt if user would like to create the default config file automatically
        warn!("Unable to read config file {}", config_file);
    }

    myshell.run().unwrap();
}
