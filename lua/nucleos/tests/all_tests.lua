package.path = "lua/?.lua;" .. package.path

local files = {
    "lua/nucleos/tests/test_compiler.lua",
    "lua/nucleos/tests/test_opts.lua",
}

-- Load each test file (each one registers suites)
for _, file in ipairs(files) do
    dofile(file)
end

-- Run all registered suites
local runner = require("nucleos.tests.runner")
runner.run()
