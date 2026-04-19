use std::path::Path;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result as EyreResult;
use mlua::{ObjectLike, Value};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

use nucleos::lua::LuaEnvironment;

#[derive(Parser)]
#[command(name = "nucleos")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reconcile config with machine state
    Apply,
    /// Show current task status
    Status,
}

fn main() -> EyreResult<()> {
    color_eyre::install()?;

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let cli = Cli::parse();

    info!("Starting nucleos");

    let lua_env = LuaEnvironment::try_new()?;
    // TODO: Accept config path as a CLI argument (PR 3)
    let config_path = Path::new("config/example/nucleos.lua");
    let tasks = lua_env.load_tasks(config_path)?;

    match cli.command {
        Commands::Apply => {
            for task in &tasks {
                if !task.opts.enabled {
                    info!(task = %task.id, "Task disabled; skipping");
                    continue;
                }
                info!(task = %task.id, "Applying task");
                // TODO: Pass stored snapshot and persist result (PR 3)
                if let Err(e) = task.module.call_method::<Value>("apply", Value::Nil) {
                    eprintln!("error applying '{}': {e}", task.id);
                }
            }
        }
        Commands::Status => {
            // TODO: Compare against stored state (PR 3)
            for task in &tasks {
                println!("- {}", task.id);
            }
        }
    }

    info!("Done");
    Ok(())
}
