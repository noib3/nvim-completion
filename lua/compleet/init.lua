local augroups = require('compleet/augroups')

local get_current_line = vim.api.nvim_get_current_line
local get_cursor = vim.api.nvim_win_get_cursor

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

local accept_completion = function()
  notify('accept_completion')
end

local cursor_moved = function()
  notify('cursor_moved')
end

local insert_left = function()
  notify('insert_left')
end

---@return boolean
local completion_menu_is_visible = function()
  return request('is_completion_menu_visible')
end

local ping = function()
  print(request('ping', 'Neovim says ping!'))
end

local select_next_completion = function()
  notify('insert_left')
end

local select_prev_completion = function()
  notify('insert_left')
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

local text_changed = function()
  notify(
    'text_changed',
    get_current_line(),
    get_cursor(0)[2]
  )
end

return {
  accept_completion = accept_completion,
  cursor_moved = cursor_moved,
  insert_left = insert_left,
  completion_menu_is_visible = completion_menu_is_visible,
  ping = ping,
  select_next_completion = select_next_completion,
  select_prev_completion = select_prev_completion,
  setup = setup,
  text_changed = text_changed,
}
