local notify = vim.rpcnotify
local get_current_line = vim.api.nvim_get_current_line
local get_cursor = vim.api.nvim_win_get_cursor
local set_keymap = vim.keymap.set

---@lhs string
---@rhs function
local set_plugmap = function(lhs, rhs)
  set_keymap('i', lhs, rhs, { noremap = true, silent = true, })
end

local accept_completion = function()
  notify(_G.compleet.channel_id, 'accept_completion')
end

local next_completion = function()
  notify(_G.compleet.channel_id, 'select_next_completion')
end

local prev_completion = function()
  notify(_G.compleet.channel_id, 'select_prev_completion')
end

local show_completions = function()
  notify(
    _G.compleet.channel_id,
    'show_completions',
    get_current_line(),
    get_cursor(0)[2]
  )
end

local setup = function()
  set_plugmap('<Plug>(compleet-accept-completion)', accept_completion)
  set_plugmap('<Plug>(compleet-next-completion)', next_completion)
  set_plugmap('<Plug>(compleet-prev-completion)', prev_completion)
  set_plugmap('<Plug>(compleet-show-completions)', show_completions)
end

return {
  setup = setup,
}
