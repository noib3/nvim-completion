local augroups = require('compleet/augroups')
local mappings = require('compleet/mappings')

local request = vim.rpcrequest

_G.compleet = {
  channel_id = nil,
}

---@param preferences  table | nil
local setup = function(preferences)
  -- If the connection has already been started return early.
  if _G.compleet.channel_id then return end

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
  _G.compleet.channel_id = vim.fn.jobstart({ path_binary }, { rpc = true })

  augroups.setup()
  mappings.setup()
end

---@return boolean
local is_menu_visible = function()
  return request(_G.compleet.channel_id, 'is_completion_menu_visible')
end

---@return boolean
local has_completions = function()
  return request(_G.compleet.channel_id, 'has_completions')
end

return {
  has_completions = has_completions,
  is_menu_visible = is_menu_visible,
  setup = setup,
}
