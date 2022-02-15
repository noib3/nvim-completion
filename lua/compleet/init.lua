local augroups = require('compleet/augroups')

local channel_id

---@param method  string
local notify = function(method, ...)
  vim.rpcnotify(channel_id, method, ...)
end

---@param method  string
---@return any
local request = function(method, ...)
  return vim.rpcrequest(channel_id, method, ...)
end

---@param preferences  table | nil
local setup = function(preferences)
  -- If the connection has already been started return early.
  if channel_id then return end

  -- The path of this file in the filesystem
  local path_self =
    vim.api.nvim_get_runtime_file('lua/compleet/init.lua', false)[1]

  -- The path of the binary executable
  local path_binary =
    vim.fn.fnamemodify(path_self, ':h:h:h')
    .. '/target/release/compleet'

  -- If the binary doesn't exist or it's not executable return early.
  if vim.fn.executable(path_binary) ~= 1 then return end

  -- Start the connection to the Rust binary
  channel_id = vim.fn.jobstart({ path_binary }, { rpc = true })

  augroups.setup()
end

return {
  notify = notify,
  request = request,
  setup = setup,
}
