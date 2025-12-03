use std::fs;

use color_eyre::Result;
use mlua::Lua;

#[test]
fn run_lua_tests() -> Result<()> {
    let lua = Lua::new();

    let test_runner_path = "lua/nucleos/tests/all_tests.lua";
    lua.load(&fs::read_to_string(test_runner_path)?).exec()?;

    Ok(())
}
