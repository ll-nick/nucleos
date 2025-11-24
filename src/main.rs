use std::fs;
use std::path::Path;

use mlua::{Lua, ObjectLike, Result, Table, UserData, UserDataMethods};

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
    let lua = Lua::new();

    register_builtins(&lua)?;

    let config: Table = lua.load(&std::fs::read_to_string("config.lua")?).eval()?;

    let tasks: Table = config.get("tasks")?;
    for pair in tasks.pairs::<String, Table>() {
        let (task_name, task_table) = pair?;

        let module: mlua::AnyUserData = task_table.get("module")?;

        println!("Running task: {}", task_name);
        module.call_method::<()>("apply", ())?;
    }

    Ok(())
}
