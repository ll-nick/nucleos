package.path = "lua/?.lua;" .. package.path

local compiler = require("nucleos.compiler")
local config = require("nucleos.tests.test_config")

local function run()
    print("=== Running Lua task compiler tests ===")

    local flat = compiler.compile(config)

    print("\n--- Flattened tasks ---")
    for _, t in ipairs(flat) do
        print(string.format("task: %-30s enabled=%s", t.id, tostring(t.opts.enabled)))
    end

    print("\n--- Running apply() for enabled tasks ---")
    for _, t in ipairs(flat) do
        if t.opts.enabled then
            t.module.apply()
        end
    end

    print("\n=== Tests complete ===\n")
end

run()
