local notify = vim.rpcnotify
local get_current_line = vim.api.nvim_get_current_line
local get_cursor = vim.api.nvim_win_get_cursor

local setup = function()
  vim.cmd([[
    augroup compleet_notify_events
      autocmd CursorMovedI * lua require('compleet/augroups').cursor_moved()
      autocmd InsertLeave  * lua require('compleet/augroups').insert_left()
      autocmd TextChangedI * lua require('compleet/augroups').text_changed()
    augroup END
  ]])
end

local cursor_moved = function()
  notify(_G.compleet.channel_id, 'cursor_moved')
end

local insert_left = function()
  notify(_G.compleet.channel_id, 'insert_left')
end

local text_changed = function()
  notify(
    _G.compleet.channel_id,
    'text_changed',
    get_current_line(),
    get_cursor(0)[2]
  )
end

return {
  setup = setup,
  cursor_moved = cursor_moved,
  insert_left = insert_left,
  text_changed = text_changed,
}
