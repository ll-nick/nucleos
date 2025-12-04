local C = {
    reset = "\27[0m",
    bold = "\27[1m",

    red = "\27[31m",
    green = "\27[32m",
    yellow = "\27[33m",
    blue = "\27[34m",
    magenta = "\27[35m",
    cyan = "\27[36m",
    gray = "\27[90m",
}

local Runner = {}

-- List of all registered test suites
local suites = {}

function Runner.suite(name, tests)
    table.insert(suites, { name = name, tests = tests })
end

function Runner.run()
    print(C.bold .. C.cyan .. "=== Running Lua Tests ===" .. C.reset)

    local passed, failed, total = 0, 0, 0

    for _, suite in ipairs(suites) do
        print("\n" .. C.bold .. "# Suite: " .. C.magenta .. suite.name .. C.reset)

        for test_name, fn in pairs(suite.tests) do
            total = total + 1
            local ok, err = pcall(fn)

            if ok then
                passed = passed + 1
                print("  " .. C.green .. "[ OK ] " .. C.reset .. test_name)
            else
                failed = failed + 1
                print("  " .. C.red .. "[FAIL] " .. C.reset .. test_name)
                print("        " .. C.gray .. err .. C.reset)
            end
        end
    end

    print("\n" .. C.bold .. C.cyan .. "=== Summary ===" .. C.reset)

    local summary = string.format("Total: %d  Passed: %d  Failed: %d", total, passed, failed)
    if failed > 0 then
        print(C.red .. summary .. C.reset)
        error("Tests failed") -- causes cargo test to fail
    else
        print(C.green .. summary .. C.reset)
    end
end

return Runner
