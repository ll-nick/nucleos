local logging = {}

---@diagnostic disable: undefined-global
-- nucleos global will be injected by the rust binary
if nucleos and nucleos.logging then
    logging = nucleos.logging
else
    -- fallback for standalone Lua/testing
    logging.debug = print
    logging.info = print
    logging.warn = print
    logging.error = print
end

return logging
