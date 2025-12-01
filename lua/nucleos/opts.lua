local Opts = {}
Opts.__index = Opts

-- default opts; more fields will be added later
local DEFAULTS = {
    enabled = true,
}

function Opts.new(tbl)
    tbl = tbl or {}
    setmetatable(tbl, Opts)

    -- fill in defaults
    for k, v in pairs(DEFAULTS) do
        if tbl[k] == nil then
            tbl[k] = v
        end
    end

    return tbl
end

function Opts.merge(parent, child)
    local result = {}

    for k, v in pairs(parent) do
        result[k] = v
    end
    for k, v in pairs(child) do
        result[k] = v
    end

    return Opts.new(result)
end

return Opts
