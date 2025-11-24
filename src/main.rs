use mlua::{Lua, Result, Table};

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
