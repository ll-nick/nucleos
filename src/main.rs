use std::path::Path;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result as EyreResult;
use mlua::{LuaSerdeExt, ObjectLike, Value};
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, fmt};

use nucleos::lua::LuaEnvironment;
use nucleos::task::{TaskState, UndoSafety};

#[derive(Parser)]
#[command(name = "nucleos")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply all tasks
    Apply,
    /// Undo all tasks
    Undo {
        #[arg(long)]
        risky: bool,
    },
    /// List all tasks
    Status,
}

#[derive(PartialEq)]
pub enum UndoMode {
    Safe,  // Only Safe
    Risky, // Allow Safe + Risky
}

fn main() -> EyreResult<()> {
    color_eyre::install()?;

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let cli = Cli::parse();

    info!("Starting nucleos");

    let lua_env = LuaEnvironment::new()?;
    let config_path = Path::new("config/example/nucleos.lua");
    let tasks = lua_env.load_tasks(config_path)?;

    match cli.command {
        Commands::Apply => {
            for task in &tasks {
                if !task.opts.enabled {
                    info!(task = %task.name, "Task is disabled; skipping");
                    continue;
                }

                info!(task = %task.name, "Applying task");
                if let Err(e) = task.module.call_method::<()>("apply", ()) {
                    error!(task = %task.name, error = %e, "Failed to apply task");
                }
            }
        }
        Commands::Undo { risky } => {
            let mode = if risky {
                UndoMode::Risky
            } else {
                UndoMode::Safe
            };

            info!(risky = %risky, "Undoing tasks");

            for task in &tasks {
                if !task.opts.enabled {
                    info!(task = %task.name, "Task disabled; skipping undo");
                    continue;
                }

                // Read undo safety
                let value: Value = task.module.call_method("undo_safety", ())?;
                let safety: UndoSafety = lua_env.lua.from_value(value)?;

                let allowed = safety == UndoSafety::Safe
                    || (safety == UndoSafety::Risky && mode == UndoMode::Risky);

                if allowed {
                    info!(task = %task.name, safety = ?safety, "Undoing task");
                    if let Err(e) = task.module.call_method::<()>("undo", ()) {
                        error!(task = %task.name, error = %e, "Failed to undo task");
                    }
                } else {
                    warn!(
                        task = %task.name,
                        safety = ?safety,
                        "Skipping undo due to safety restrictions"
                    );
                }
            }
        }
        Commands::Status => {
            info!("Listing task status");

            for task in &tasks {
                let value: Value = task.module.call_method("state", ())?;
                let state: TaskState = lua_env.lua.from_value(value)?;

                println!("- {}: {:?}", task.name, state);
            }
        }
    }

    info!("Done");
    Ok(())
}
