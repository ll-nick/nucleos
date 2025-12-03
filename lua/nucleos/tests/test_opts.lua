local Opts = require("nucleos.opts")
local utils = require("nucleos.tests.utils")
local runner = require("nucleos.tests.runner")

local opts_tests = {}

opts_tests["defaults"] = function()
    local o = Opts.new()
    utils.assert_true(o.enabled)
end

opts_tests["merge_and_gate"] = function()
    local o1 = Opts.new({ enabled = true })
    local o2 = Opts.new({ enabled = false })
    o1:merge(o2)
    utils.assert_false(o1.enabled)
end

runner.suite("Opts tests", opts_tests)
