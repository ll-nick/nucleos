use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result as EyreResult;
use mlua::{
    AnyUserData, Lua, LuaSerdeExt, ObjectLike, Result as LuaResult, Table, UserData,
    UserDataMethods, Value,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{EnvFilter, fmt};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    Applied,
    OutOfDate,
    NotApplied,
    Stateless,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoSafety {
    /// Fully revertible without side effects
    Safe,

    /// Reversible but may have side effects
    Risky,

    /// Cannot be undone automatically
    Impossible,
}

#[derive(PartialEq)]
pub enum UndoMode {
    Safe,  // Only Safe
    Risky, // Allow Safe + Risky
}

pub trait Module {
    fn apply(&self) -> LuaResult<()>;
    fn undo(&self) -> LuaResult<()>;
    fn undo_safety(&self) -> UndoSafety;
    fn state(&self) -> LuaResult<TaskState>;
}

pub struct Echo {
    pub message: String,
}

impl Module for Echo {
    fn apply(&self) -> LuaResult<()> {
        println!("{}", self.message);
        Ok(())
    }

    fn undo(&self) -> LuaResult<()> {
        Ok(())
    }

    fn undo_safety(&self) -> UndoSafety {
        UndoSafety::Safe
    }

    fn state(&self) -> LuaResult<TaskState> {
        Ok(TaskState::Stateless)
    }
}

pub struct File {
    pub path: String,
}

impl Module for File {
    fn apply(&self) -> LuaResult<()> {
        fs::write(&self.path, b"Created by nucleos")?;
        println!("File created: {}", self.path);
        Ok(())
    }

    fn undo(&self) -> LuaResult<()> {
        if Path::new(&self.path).exists() {
            fs::remove_file(&self.path)?;
            println!("File removed: {}", self.path);
        }
        Ok(())
    }

    fn undo_safety(&self) -> UndoSafety {
        UndoSafety::Safe
    }

    fn state(&self) -> LuaResult<TaskState> {
        let exists = Path::new(&self.path).exists();
        if exists {
            Ok(TaskState::Applied)
        } else {
            Ok(TaskState::NotApplied)
        }
    }
}

impl UserData for Echo {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("apply", |_, this, ()| this.apply());
        methods.add_method("undo", |_, this, ()| this.undo());
        methods.add_method("undo_safety", |lua, this, ()| {
            let s = this.undo_safety();
            lua.to_value(&s)
        });
        methods.add_method("state", |lua, this, ()| {
            let s = this.state()?;
            lua.to_value(&s)
        });
    }
}

impl UserData for File {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("apply", |_, this, ()| this.apply());
        methods.add_method("undo", |_, this, ()| this.undo());
        methods.add_method("undo_safety", |lua, this, ()| {
            let s = this.undo_safety();
            lua.to_value(&s)
        });
        methods.add_method("state", |lua, this, ()| {
            let s = this.state()?;
            lua.to_value(&s)
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskOpts {
    pub enabled: bool,
}

impl Default for TaskOpts {
    fn default() -> Self {
        TaskOpts { enabled: true }
    }
}

fn register_nucleos(lua: &Lua) -> EyreResult<()> {
    let nucleos = lua.create_table()?;
    lua.globals().set("nucleos", nucleos)?;
    Ok(())
}

fn register_logging(lua: &Lua) -> EyreResult<()> {
    let logging = lua.create_table()?;

    let debug = lua.create_function(|_, msg: String| {
        debug!(target: "lua", "{msg}");
        Ok(())
    })?;
    logging.set("debug", debug)?;

    let info = lua.create_function(|_, msg: String| {
        info!(target: "lua", "{msg}");
        Ok(())
    })?;
    logging.set("info", info)?;

    let warn = lua.create_function(|_, msg: String| {
        warn!(target: "lua", "{msg}");
        Ok(())
    })?;
    logging.set("warn", warn)?;

    let error = lua.create_function(|_, msg: String| {
        error!(target: "lua", "{msg}");
        Ok(())
    })?;
    logging.set("error", error)?;

    let nucleos = lua.globals().get::<Table>("nucleos")?;
    nucleos.set("logging", logging)?;

    Ok(())
}

fn register_builtins(lua: &Lua) -> EyreResult<()> {
    let builtin = lua.create_table()?;

    let echo = lua.create_function(|_, opts: Table| {
        let message: String = opts.get("message")?;
        Ok(Echo { message })
    })?;
    builtin.set("echo", echo)?;

    let file = lua.create_function(|_, opts: Table| {
        let path: String = opts.get("path")?;
        Ok(File { path })
    })?;
    builtin.set("file", file)?;

    let nucleos = lua.globals().get::<Table>("nucleos")?;
    nucleos.set("builtin", builtin)?;

    Ok(())
}

fn main() -> EyreResult<()> {
    color_eyre::install()?;

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let cli = Cli::parse();

    info!("Starting nucleos");

    let lua = Lua::new();
    register_nucleos(&lua)?;
    register_logging(&lua)?;
    register_builtins(&lua)?;

    // Preload dependencies
    let package: Table = lua.globals().get("package")?;
    let preload: Table = package.get("preload")?;

    let logging_src = include_str!("../lua/nucleos/logging.lua");
    let opts_src = include_str!("../lua/nucleos/opts.lua");
    let utils_src = include_str!("../lua/nucleos/utils.lua");

    preload.set("nucleos.logging", lua.load(logging_src).into_function()?)?;
    preload.set("nucleos.utils", lua.load(utils_src).into_function()?)?;
    preload.set("nucleos.opts", lua.load(opts_src).into_function()?)?;

    // Load task compiler
    let compiler_src = include_str!("../lua/nucleos/compiler.lua");
    let compiler: mlua::Table = lua.load(compiler_src).eval()?;

    let config_path = "config/example/nucleos.lua";
    info!(path = config_path, "Loading Lua config");
    let config: Table = lua.load(&fs::read_to_string(config_path)?).eval()?;

    let tasks_table: mlua::Table = compiler.call_method("compile", (config,))?;

    match cli.command {
        Commands::Apply => {
            for pair in tasks_table.pairs::<String, Table>() {
                let (name, task_table) = pair?;
                let module: mlua::AnyUserData = task_table.get("module")?;

                let opts_table: Option<Table> = task_table.get("opts").ok();
                let opts: TaskOpts = if let Some(table) = opts_table {
                    lua.from_value(Value::Table(table))?
                } else {
                    TaskOpts::default()
                };

                if !opts.enabled {
                    info!(task = %name, "Task is disabled; skipping");
                    continue;
                }

                info!(task = %name, "Applying task");
                if let Err(e) = module.call_method::<()>("apply", ()) {
                    error!(task = %name, error = %e, "Failed to apply task");
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

            for pair in tasks_table.pairs::<String, Table>() {
                let (name, task_table) = pair?;
                let module: AnyUserData = task_table.get("module")?;
                let value: Value = module.call_method("undo_safety", ())?;
                let safety: UndoSafety = lua.from_value(value)?;

                let opts_table: Option<Table> = task_table.get("opts").ok();
                let opts: TaskOpts = if let Some(table) = opts_table {
                    lua.from_value(Value::Table(table))?
                } else {
                    TaskOpts::default()
                };

                if !opts.enabled {
                    info!(task = %name, "Task is disabled; skipping undo");
                    continue;
                }

                let allowed = safety == UndoSafety::Safe
                    || (safety == UndoSafety::Risky && mode == UndoMode::Risky);

                if allowed {
                    info!(task = %name, safety = ?safety, "Undoing task");
                    if let Err(e) = module.call_method::<()>("undo", ()) {
                        error!(task = %name, error = %e, "Failed to undo task");
                    }
                } else {
                    warn!(task = %name, safety = ?safety, "Skipping undo due to safety restrictions");
                };
            }
        }
        Commands::Status => {
            info!("Listing task status");
            for pair in tasks_table.pairs::<String, Table>() {
                let (name, task_table) = pair?;
                let module: AnyUserData = task_table.get("module")?;
                let value: Value = module.call_method("state", ())?;
                let state: TaskState = lua.from_value(value)?;
                println!("- {}: {:?}", name, state);
            }
        }
    }

    info!("Done");
    Ok(())
}
