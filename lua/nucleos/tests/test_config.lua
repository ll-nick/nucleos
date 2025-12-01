local M = {}

-- simple mock modules
local function mk_module(name)
    return {
        apply = function()
            print("apply " .. name)
        end,
        undo = function()
            print("undo  " .. name)
        end,
    }
end

M.tasks = {
    top_task = {
        module = mk_module("top_task"),
    },

    groupA = {
        opts = { enabled = true },

        task1 = {
            module = mk_module("groupA.task1"),
        },

        subgroup = {
            opts = { enabled = false },

            leaf = {
                module = mk_module("groupA.subgroup.leaf"),
                opts = { enabled = true },
            },
        },
    },
}

return M
