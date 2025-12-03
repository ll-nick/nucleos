local merging = require("nucleos.merging")
local logging = require("nucleos.logging")

local Opts = {}
Opts.__index = Opts

local schema = {
    enabled = { type = "boolean", default = true, merge = merging.and_gate },
}

local function validate_opts(opts)
    for k, v in pairs(opts) do
        local info = schema[k]
        if info == nil then
            logging.warn(string.format("Unknown opts key: '%s'. This value will be ignored.", k))
            opts[k] = nil
        end
        if info ~= nil and type(v) ~= info.type then
            error(string.format("Invalid type for option '%s': expected %s, got %s", k, info.type, type(v)))
        end
    end

    return opts
end

function Opts.new(tbl)
    tbl = tbl or {}
    setmetatable(tbl, Opts)

    tbl = validate_opts(tbl)

    for k, info in pairs(schema) do
        if tbl[k] == nil then
            tbl[k] = info.default
        end
    end

    return tbl
end

function Opts:merge(child)
    for key, info in pairs(schema) do
        if child[key] ~= nil and info.merge then
            self[key] = info.merge(self[key], child[key])
        elseif child[key] ~= nil then
            error(string.format("No merge function defined for option '%s'", key))
        end
    end

    return self
end

return Opts
