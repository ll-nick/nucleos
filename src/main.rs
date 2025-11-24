use std::fs;
use std::path::Path;

use mlua::{Lua, Result, Table};

pub trait Module {
    fn apply(&self) -> Result<()>;
    fn undo(&self) -> Result<()>;
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
        // nothing to undo for echo
        Ok(())
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
}

fn main() -> Result<()> {
    // Initialize Lua
    let lua = Lua::new();

    // Load the config file
    let config: Table = lua.load(&std::fs::read_to_string("config.lua")?).eval()?;

    // Access fields
    let tasks: Table = config.get("tasks")?;

    for pair in tasks.pairs::<String, Table>() {
        let (task_name, task_table) = pair?;
        let module: String = task_table.get("module")?;
        let options: Table = task_table.get("options")?;

        println!("Task Name: {}", task_name);
        println!("Module: {}", module);
        println!("Options:");

        for opt_pair in options.pairs::<String, mlua::Value>() {
            let (opt_key, opt_value) = opt_pair?;
            println!("  {}: {:?}", opt_key, opt_value);
        }

        println!();
    }

    Ok(())
}
