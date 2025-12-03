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

local function assert_nil(value, msg)
    if value ~= nil then
        error(msg or string.format("Assertion failed: expected nil but got %s", fmt(value)), 2)
    end
end

local function assert_true(value, msg)
    if value ~= true then
        error(msg or string.format("Assertion failed: expected true but got %s", fmt(value)), 2)
    end
end

local function assert_false(value, msg)
    if value ~= false then
        error(msg or string.format("Assertion failed: expected false but got %s", fmt(value)), 2)
    end
end

local function assert_equal(actual, expected, msg)
    if actual ~= expected then
        error(msg or string.format("Assertion failed: expected %s but got %s", fmt(expected), fmt(actual)), 2)
    end
end

local function assert_match(str, pattern, msg)
    if not string.match(str, pattern) then
        error(msg or string.format("Assertion failed: string '%s' does not match pattern '%s'", str, pattern), 2)
    end
end

M = {
    assert_nil = assert_nil,
    assert_true = assert_true,
    assert_false = assert_false,
    assert_equal = assert_equal,
    assert_match = assert_match,
}

return M
