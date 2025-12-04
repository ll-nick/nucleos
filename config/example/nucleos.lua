return {
    settings = {
        -- log_file = "/var/log/nucleos.log", -- TODO
        -- data_dir = "XDG_DATA_HOME/nucleos", -- TODO
        -- parallel_tasks = 4, -- TODO
        -- tags = { "default" = true, "tag1" = true, "tag2" = false }, -- TODO -- With default = true, run all tasks except explicitly disabled ones, or with default = false, run only explicitly enabled ones
        -- merging_strategy = { name = "lowest_level", ...} -- TODO
    },
    tasks = {
        -- Tasks can be grouped into named groups which also prevents name collisions
        -- Below task would be named "some_group.test_task"
        {
            "some_group",
            opts = {}, -- Groups take the same options table as tasks. Merging group options into task options needs to be defined (e.g., enabled at group level disables all tasks in group), see merging_strategie global setting
            tasks = {
                {
                    "test_task",
                    module = nucleos.builtin.file({
                        path = "/tmp/test.txt",
                    }),
                    -- The opts table can be defined for every individual task or for task groups
                    -- opts tables get merged using different strategies
                    -- e.g. tags are merged as a union, user is taken by the lowest level definition (task opts beats group opts)
                    opts = {
                        -- name = "A human friendly name", -- TODO
                        -- description = "A description of the task", -- TODO

                        enabled = true,
                        -- enabled = { apply = true, undo = true }, -- TODO
                        -- enabled = function() return true end, -- TODO

                        -- undo_mode = "safe|unsafe" -- TODO
                        -- depends_on = { "another_task" }, -- TODO
                        -- priority = 10, -- TODO
                        -- user = "root", -- TODO
                        -- environment = { VAR1 = "value1", VAR2 = "value2" }, -- TODO
                        -- tags = { "tag1", "tag2" }, -- TODO
                        -- dry_run = true, -- TODO
                        -- log_level = "info", -- TODO

                        -- Callbacks
                        -- before_apply = function() end, TODO
                        -- after_apply = function() end, TODO
                        -- before_undo = function() end, TODO
                        -- after_undo = function() end, TODO
                        -- on_error = function() end, -- TODO
                    },
                },
                {
                    "another_task ",
                    module = nucleos.builtin.echo({
                        message = "Hello, World!",
                    }),
                },
            },
            -- TODO: Structured config by merging tasks from multiple sources
            -- Import all tasks from Lua files in the specified directory
            -- Each Lua file inside those directories should return a list of task definitions, optionally further grouped and with options
            -- { import = "path/to/some/directory" } -- A sane default would probably be "tasks/"
        },
    },
}
