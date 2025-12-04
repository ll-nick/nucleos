use std::fs;
use std::path::Path;

use color_eyre::eyre::Result;
use mlua::{Lua, LuaSerdeExt, ObjectLike, Table, Value};
use tracing::{debug, error, info, warn};

use crate::module::{Echo, File};
use crate::task::{Task, TaskOpts};

fn register_nucleos(lua: &Lua) -> Result<()> {
    let nucleos = lua.create_table()?;
    let builtin = lua.create_table()?;
    nucleos.set("builtin", builtin)?;
    lua.globals().set("nucleos", nucleos)?;
    Ok(())
}

fn register_logging(lua: &Lua) -> Result<()> {
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

fn load_nucleos_modules(lua: &Lua) -> Result<()> {
    let package: Table = lua.globals().get("package")?;
    let preload: Table = package.get("preload")?;

    let logging_src = include_str!("../lua/nucleos/logging.lua");
    let merging_src = include_str!("../lua/nucleos/merging.lua");
    let opts_src = include_str!("../lua/nucleos/opts.lua");
    let utils_src = include_str!("../lua/nucleos/utils.lua");
    let compiler_src = include_str!("../lua/nucleos/compiler.lua");

    preload.set("nucleos.logging", lua.load(logging_src).into_function()?)?;
    preload.set("nucleos.merging", lua.load(merging_src).into_function()?)?;
    preload.set("nucleos.utils", lua.load(utils_src).into_function()?)?;
    preload.set("nucleos.opts", lua.load(opts_src).into_function()?)?;
    preload.set("nucleos.compiler", lua.load(compiler_src).into_function()?)?;

    Ok(())
}

fn register_builtin_modules(lua: &Lua) -> Result<()> {
    // TODO: Can we register modules in the module files themselves?
    let nucleos: Table = lua.globals().get("nucleos")?;
    let builtin: Table = nucleos.get("builtin")?;

    // echo
    let echo_ctor = lua.create_function(|_, opts: Table| {
        let message: String = opts.get("message")?;
        Ok(Echo { message })
    })?;
    builtin.set("echo", echo_ctor)?;

    // file
    let file_ctor = lua.create_function(|_, opts: Table| {
        let path: String = opts.get("path")?;
        Ok(File { path })
    })?;
    builtin.set("file", file_ctor)?;

    Ok(())
}

pub struct LuaEnvironment {
    pub lua: Lua,
}

impl LuaEnvironment {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();

        register_nucleos(&lua)?;
        register_logging(&lua)?;
        load_nucleos_modules(&lua)?;
        register_builtin_modules(&lua)?;

        Ok(LuaEnvironment { lua })
    }

    pub fn load_tasks(&self, config_path: &Path) -> Result<Vec<Task>> {
        let config: Table = self.lua.load(&fs::read_to_string(config_path)?).eval()?;

        let compiler: mlua::Table = self.lua.load("return require('nucleos.compiler')").eval()?;
        let tasks_table: mlua::Table = compiler.call_method("compile", (config,))?;

        let mut tasks = Vec::new();

        for pair in tasks_table.pairs::<String, Table>() {
            let (name, task_table) = pair?;
            let module = task_table.get("module")?;

            // opts is optional
            let opts_table: Option<Table> = task_table.get("opts").ok();
            let opts = match opts_table {
                Some(o) => self.lua.from_value(Value::Table(o))?,
                None => TaskOpts::default(),
            };

            tasks.push(Task { name, module, opts });
        }

        Ok(tasks)
    }
}
