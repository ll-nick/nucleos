local M = {}

-- Merges two boolean values with logical AND
local function and_gate(a, b)
    return a and b
end

M.and_gate = and_gate

return M
