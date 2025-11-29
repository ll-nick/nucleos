return {
    settings = {
        -- log_level = "info", -- TODO
        -- log_file = "/var/log/nucleos.log", -- TODO
        -- data_dir = "XDG_DATA_HOME/nucleos", -- TODO
        -- parallel_tasks = 4, -- TODO
        -- dry_run = false, -- TODO
        -- cache_enabled = true, -- TODO -- No need to run state checks for each task if we just read the last state from file
        -- tags = { "default" = true, "tag1" = true, "tag2" = false }, -- TODO -- With default = true, run all tasks except explicitly disabled ones, or with default = false, run only explicitly enabled ones
        -- merging_strategie = { name = "lowest_level", ...} -- TODO
    },
    tasks = {
        -- TODO: Tasks can be grouped into named groups which also prevents name collisions
        -- Below task would be named "some_group.test_task"
        -- some_group = {
        -- opts = {} -- TODO -- Groups take the same options table as tasks. Merging group options into task options needs to be defined (e.g., enabled at group level disables all tasks in group), see merging_strategie global setting
        test_task = {
            module = nucleos.builtin.file({
                path = "/tmp/test.txt",
            }),
            -- The opts table can be defined for every individual task or for task groups
            -- opts tables get merged using different strategies
            -- e.g. tags are merged as a union, user is taken by the lowest level definition (task opts beats group opts)
            opts = {
                -- name = "A human friendly name", -- TODO
                -- description = "A description of the task", -- TODO

                -- enabled = true, -- TODO
                -- enabled = { apply = true, undo = true }, -- TODO
                -- enabled = function() return true end, -- TODO

                -- undo_mode = "safe|unsafe" -- TODO
                -- depends_on = { "another_task" }, -- TODO
                -- priority = 10, -- TODO
                -- user = "root", -- TODO
                -- environment = { VAR1 = "value1", VAR2 = "value2" }, -- TODO
                -- tags = { "tag1", "tag2" }, -- TODO
                -- dry_run = true, -- TODO

                -- Callbacks
                -- before_apply = function() end, TODO
                -- after_apply = function() end, TODO
                -- before_undo = function() end, TODO
                -- after_undo = function() end, TODO
                -- on_error = function() end, -- TODO
            },
        },
        another_task = {
            module = nucleos.builtin.echo({
                message = "Hello, World!",
            }),
        },
        -- },
        -- TODO: Structured config by merging tasks from multiple sources
        -- Import all tasks from Lua files in the specified directory
        -- Each directory inside the given directory represents a task group
        -- Each Lua file inside those directories represents a task
        -- Each directory may contain a special "opts.lua" file which returns an options table configuring the group defined by the parent directory
        -- E.g. "tasks/some_group/opts.lua" defines options for the "some_group" task group
        -- { import = "path/to/some/directory" } -- A sane default would probably be "tasks/"
    },
}
