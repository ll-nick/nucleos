local logging = require("nucleos.logging")
local Opts = require("nucleos.opts")
local runner = require("nucleos.tests.runner")
local utils = require("nucleos.tests.utils")

local opts_tests = {}

opts_tests["defaults"] = function()
    local o = Opts.new()
    utils.assert_true(o.enabled)
end

opts_tests["unknown_key_warns_and_ignores"] = function()
    local warned = false
    -- Temporarily override logging.warn to capture the message
    local orig_warn = logging.warn
    logging.warn = function(msg)
        warned = true
        utils.assert_match(msg, "Unknown opts key")
    end

    local o = Opts.new({ enabled = true, foobar = 123 })
    utils.assert_true(o.enabled)
    utils.assert_nil(o.foobar)
    utils.assert_true(warned)

    -- Restore original logging.warn
    logging.warn = orig_warn
end

opts_tests["invalid_type_error"] = function()
    local ok, err = pcall(function()
        Opts.new({ enabled = "not_a_boolean" })
    end)
    utils.assert_false(ok)
    utils.assert_match(err, "Invalid type for option 'enabled'")
end

opts_tests["merge_and_gate"] = function()
    local o1 = Opts.new({ enabled = true })
    local o2 = Opts.new({ enabled = false })
    o1:merge(o2)
    utils.assert_false(o1.enabled)
end

runner.suite("Opts tests", opts_tests)
