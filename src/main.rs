use std::fs;
use std::path::Path;

use clap::{Parser, Subcommand};
use mlua::{Lua, ObjectLike, Result, Table, UserData, UserDataMethods};

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
    Undo,
    /// List all tasks
    Status,
}

pub enum TaskState {
    Applied,
    OutOfDate,
    NotApplied,
    Stateless,
}

pub enum UndoSafety {
    /// Fully revertible without side effects
    Safe,

    /// Reversible but may have side effects
    Risky,

    /// Cannot be undone automatically
    Impossible,
}

pub enum UndoMode {
    Safe,  // Only Safe
    Risky, // Allow Safe + Risky
}

pub trait Module {
    fn apply(&self) -> Result<()>;
    fn undo(&self) -> Result<()>;
    fn undo_safety(&self) -> UndoSafety;
    fn state(&self) -> Result<TaskState>;
}

pub struct Echo {
    pub message: String,
}

impl Module for Echo {
    fn apply(&self) -> Result<()> {
        println!("{}", self.message);
        Ok(())
    }

    fn undo(&self) -> Result<()> {
        Ok(())
    }

    fn undo_safety(&self) -> UndoSafety {
        UndoSafety::Safe
    }

    fn state(&self) -> Result<TaskState> {
        Ok(TaskState::Stateless)
    }
}

pub struct File {
    pub path: String,
}

impl Module for File {
    fn apply(&self) -> Result<()> {
        fs::write(&self.path, b"Created by nucleos")?;
        println!("File created: {}", self.path);
        Ok(())
    }

    fn undo(&self) -> Result<()> {
        if Path::new(&self.path).exists() {
            fs::remove_file(&self.path)?;
            println!("File removed: {}", self.path);
        }
        Ok(())
    }

    fn undo_safety(&self) -> UndoSafety {
        UndoSafety::Risky
    }

    fn state(&self) -> Result<TaskState> {
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
    }
}

impl UserData for File {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("apply", |_, this, ()| this.apply());
        methods.add_method("undo", |_, this, ()| this.undo());
    }
}

fn register_builtins(lua: &Lua) -> Result<()> {
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

    let nucleos = lua.create_table()?;
    nucleos.set("builtin", builtin.clone())?;
    lua.globals().set("nucleos", nucleos)?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let lua = Lua::new();
    register_builtins(&lua)?;

    let config: Table = lua.load(&std::fs::read_to_string("config.lua")?).eval()?;
    let tasks_table: Table = config.get("tasks")?;

    match cli.command {
        Commands::Apply => {
            for pair in tasks_table.pairs::<String, Table>() {
                let (name, task_table) = pair?;
                let module: mlua::AnyUserData = task_table.get("module")?;
                println!("Applying task: {}", name);
                module.call_method::<()>("apply", ())?;
            }
        }
        Commands::Undo => {
            for pair in tasks_table.pairs::<String, Table>() {
                let (name, task_table) = pair?;
                let module: mlua::AnyUserData = task_table.get("module")?;
                println!("Undoing task: {}", name);
                module.call_method::<()>("undo", ())?;
            }
        }
        Commands::Status => {
            println!("Tasks loaded:");
            for pair in tasks_table.pairs::<String, Table>() {
                let (name, task_table) = pair?;
                let module: mlua::AnyUserData = task_table.get("module")?;
                println!(" - {}", name);
            }
        }
    }

    Ok(())
}
