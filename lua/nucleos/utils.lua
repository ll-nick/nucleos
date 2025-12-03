local M = {}

local function table_to_string(t, indent)
    assert(type(t) == "table", "Expected a table")

    indent = indent or 0
    local pad = string.rep("  ", indent)
    local result = "{\n"

    for k, v in pairs(t) do
        local key = tostring(k)
        result = result .. pad .. "  " .. key .. " = "

        if type(v) == "table" then
            result = result .. table_to_string(v, indent + 1)
        else
            result = result .. tostring(v)
        end

        result = result .. ",\n"
    end

    return result .. pad .. "}"
end

M.table_to_string = table_to_string

return M
