use std::fs;

use color_eyre::Result;

use nucleos::lua::LuaEnvironment;

#[test]
fn run_lua_tests() -> Result<()> {
    let lua_env = LuaEnvironment::try_new()?;

    let test_runner_path = "lua/nucleos/tests/all_tests.lua";
    lua_env
        .lua
        .load(&fs::read_to_string(test_runner_path)?)
        .exec()?;

    Ok(())
}
