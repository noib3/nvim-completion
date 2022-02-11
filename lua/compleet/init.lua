local channel_id

-- The path of this file in the filesystem
local path_self =
  vim.api.nvim_get_runtime_file('lua/compleet/init.lua', false)[1]

-- The path of the binary executable
local path_binary =
  vim.fn.fnamemodify(path_self, ':h:h:h')
  .. '/target/release/compleet'

---@param channel  number
---@param method  string
---@return any
local notify = function(channel, method, ...)
  vim.rpcnotify(channel, method, ...)
end

---@param channel  number
---@param method  string
---@return any
local request = function(channel, method, ...)
  return vim.rpcrequest(channel, method, ...)
end

-- Start the connection to the Rust binary
local start = function()
  -- If the connection has already been started return early.
  if channel_id then return end

  -- If the binary doesn't exist or it's not executable return early.
  if vim.fn.executable(path_binary) ~= 1 then return end

  channel_id = vim.fn.jobstart({ path_binary }, { rpc = true })
end

local ping = function()
  print(request(channel_id, 'ping', 'Neovim says ping!'))
end

return {
  start = start,
  ping = ping,
}
