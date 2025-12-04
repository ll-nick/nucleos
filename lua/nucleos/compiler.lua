local log = require("nucleos.logging")
local Opts = require("nucleos.opts")
local utils = require("nucleos.utils")

local Compiler = {}

local function is_task(entry)
    return type(entry) == "table" and entry.module ~= nil
end

local function is_group(entry)
    return type(entry) == "table" and entry.tasks ~= nil
end

local function extract_name(entry, is_group, index)
    local positional = entry[1]
    local named = entry.name

    if positional and named then
        if is_task(entry) then
            error("Pick a style: Task cannot have both positional and named name: " .. tostring(entry.name))
        else
            error("Pick a style: Group cannot have both positional and named name: " .. tostring(entry.name))
        end
    end

    if positional then
        return positional
    elseif named then
        return named
    end

    if is_group then
        return "group_" .. tostring(index)
    else
        return "task_" .. tostring(index)
    end
end

local function process_group(prefix, list, inherited_opts, out)
    inherited_opts = inherited_opts or Opts.new()
    out = out or {}

    local group_index = 0
    local task_index = 0

    for i, entry in ipairs(list) do
        local is_group_entry = is_group(entry)
        local is_task_entry = is_task(entry)

        local name
        if is_group_entry then
            group_index = group_index + 1
            name = extract_name(entry, true, group_index)
        elseif is_task_entry then
            task_index = task_index + 1
            name = extract_name(entry, false, task_index)
        else
            error("Invalid entry in tasks: must be a task or group, got: " .. tostring(entry))
        end

        local id = prefix and (prefix .. "." .. name) or name

        opts = Opts.new(entry.opts):merge(inherited_opts)

        if is_task(entry) then
            table.insert(out, {
                id = id,
                module = entry.module,
                opts = opts,
            })
        else
            process_group(id, entry.tasks, opts, out)
        end
    end

    return out
end

function Compiler:compile(config)
    local tasks = config.tasks or {}
    local flat = process_group(nil, tasks)

    log.debug("Compiled tasks:\n" .. utils.table_to_string(flat))
    return flat
end

return Compiler
