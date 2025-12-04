local Compiler = require("nucleos.compiler")
local runner = require("nucleos.tests.runner")
local utils = require("nucleos.tests.utils")

local compiler_tests = {}

compiler_tests["flat_tasks"] = function()
    local config = {
        tasks = {
            { "task1", module = {} },
            { name = "task2", module = {} },
        },
    }

    local flat = Compiler:compile(config)

    utils.assert_equal(#flat, 2)
    utils.assert_equal(flat[1].id, "task1")
    utils.assert_equal(flat[2].id, "task2")
end

compiler_tests["nested_groups"] = function()
    local config = {
        tasks = {
            {
                "group1",
                tasks = {
                    { "task1", module = {} },
                    { "task2", module = {} },
                    {
                        "subgroup",
                        opts = { enabled = false },
                        tasks = {
                            { "task3", module = {}, opts = { enabled = true } },
                        },
                    },
                },
            },
        },
    }

    local flat = Compiler:compile(config)

    utils.assert_equal(#flat, 3)

    -- Order
    utils.assert_equal(flat[1].id, "group1.task1")
    utils.assert_equal(flat[2].id, "group1.task2")
    utils.assert_equal(flat[3].id, "group1.subgroup.task3")

    -- opts inheritance
    utils.assert_true(flat[1].opts.enabled)
    utils.assert_true(flat[2].opts.enabled)
    utils.assert_false(flat[3].opts.enabled)
end

compiler_tests["auto_generated_names"] = function()
    local config = {
        tasks = {
            { module = {} },
            { module = {} },
            { tasks = {
                { module = {} },
            } },
        },
    }

    local flat = Compiler:compile(config)

    utils.assert_equal(flat[1].id, "task_1")
    utils.assert_equal(flat[2].id, "task_2")
    utils.assert_equal(flat[3].id, "group_1.task_1")
end

compiler_tests["empty_tasks"] = function()
    local config = { tasks = {} }
    local flat = Compiler:compile(config)
    utils.assert_equal(#flat, 0)
end

runner.suite("Compiler Tests", compiler_tests)
