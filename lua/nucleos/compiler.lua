local Opts = require("nucleos.opts")

local Compiler = {}

-- recursively walk structure and build flat task list
local function process_group(prefix, node, inherited_opts, out)
    inherited_opts = inherited_opts or Opts.new()
    out = out or {}

    for key, value in pairs(node) do
        if key == "opts" then
            -- merge group opts into inherited opts
            inherited_opts = Opts.new(Opts.merge(inherited_opts, value))
        else
            local id = prefix and (prefix .. "." .. key) or key

            if type(value) == "table" and value.module ~= nil then
                -- it's a task
                local opts = value.opts and Opts.merge(inherited_opts, value.opts) or inherited_opts

                table.insert(out, {
                    id = id,
                    module = value.module,
                    opts = opts,
                })
            elseif type(value) == "table" then
                -- it's a group
                process_group(id, value, inherited_opts, out)
            end
        end
    end

    return out
end

function Compiler.compile(config)
    local tasks = config.tasks or {}
    local flat = process_group(nil, tasks)
    return flat
end

return Compiler
