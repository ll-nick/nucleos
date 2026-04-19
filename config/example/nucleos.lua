return {
    settings = {
        -- log_file = "/var/log/nucleos.log", -- TODO
        -- data_dir = "XDG_DATA_HOME/nucleos", -- TODO
        -- parallel_tasks = 4, -- TODO
        -- tags = { default = true, tag1 = true, tag2 = false }, -- TODO
        -- merging_strategy = { name = "lowest_level" }, -- TODO
    },
    tasks = {
        -- Tasks can be grouped into named groups which also prevents name collisions.
        -- The task below would have id "dotfiles.zshrc".
        {
            "dotfiles",
            -- Groups take the same opts table as tasks. Options are merged into child
            -- tasks (e.g. enabled = false on a group disables all tasks inside it).
            opts = {},
            tasks = {
                {
                    "zshrc",
                    module = nucleos.builtin.copy({
                        -- src: path to the source file (absolute, or relative to config dir).
                        -- content: inline string content (mutually exclusive with src).
                        src = "/tmp/nucleos-example-source.txt",
                        dest = "/tmp/nucleos-example-dest.txt",
                        -- force = true, -- overwrite dest if it already exists (Imperfect undo)
                        -- mode = "0644",  -- TODO
                        -- owner = "user", -- TODO
                        -- group = "group", -- TODO
                    }),
                    opts = {
                        -- name = "A human friendly name", -- TODO
                        -- description = "Description of what this task does", -- TODO

                        enabled = true,
                        -- enabled = { apply = true, undo = true }, -- TODO
                        -- enabled = function() return true end, -- TODO

                        -- depends_on = { "another_task" }, -- TODO
                        -- priority = 10, -- TODO
                        -- user = "root", -- TODO
                        -- environment = { VAR1 = "value1", VAR2 = "value2" }, -- TODO
                        -- tags = { "tag1", "tag2" }, -- TODO

                        -- Callbacks
                        -- before_apply = function() end, -- TODO
                        -- after_apply = function() end, -- TODO
                        -- before_undo = function() end, -- TODO
                        -- after_undo = function() end, -- TODO
                        -- on_error = function() end, -- TODO
                    },
                },
                {
                    "symlink_example",
                    module = nucleos.builtin.link({
                        src = "/tmp/nucleos-example-source.txt",
                        dest = "/tmp/nucleos-example-link.txt",
                        -- force = true,
                    }),
                },
                {
                    "greeting",
                    module = nucleos.builtin.echo({
                        message = "Hello from nucleos!",
                    }),
                },
            },
            -- TODO: Import tasks from Lua files in a directory
            -- { import = "tasks/" },
        },
    },
}
