local M = {}

-- Internal: format a value for display
local function fmt(val)
    if type(val) == "table" then
        local parts = {}
        for k, v in pairs(val) do
            table.insert(parts, string.format("%s=%s", tostring(k), tostring(v)))
        end
        return "{" .. table.concat(parts, ", ") .. "}"
    else
        return tostring(val)
    end
end

-- Assertions
local function assert_equal(actual, expected, msg)
    if actual ~= expected then
        error(msg or string.format("Assertion failed: expected %s but got %s", fmt(expected), fmt(actual)))
    end
end

local function assert_true(value, msg)
    if value ~= true then
        error(msg or string.format("Assertion failed: expected true but got %s", fmt(value)))
    end
end

local function assert_false(value, msg)
    if value ~= false then
        error(msg or string.format("Assertion failed: expected false but got %s", fmt(value)))
    end
end

M = {
    assert_equal = assert_equal,
    assert_true = assert_true,
    assert_false = assert_false,
}

return M
