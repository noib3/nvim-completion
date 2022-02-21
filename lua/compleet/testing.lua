local compleet = require("compleet")

local request = vim.rpcrequest
local set_keymap = vim.keymap.set

local ping = function()
  print(request(_G.compleet.channel_id, "ping", "Neovim says ping!"))
end

---@return string
local i_Tab = function()
  return (compleet.is_menu_visible() and "<Plug>(compleet-next-completion)")
    or (compleet.has_completions() and "<Plug>(compleet-show-completions)")
    or "<Tab>"
end

---@return string
local i_STab = function()
  return compleet.is_menu_visible() and "<Plug>(compleet-prev-completion)"
    or "<S-Tab>"
end

---@return string
local i_CR = function()
  return compleet.is_completion_selected()
      and "<Plug>(compleet-accept-completion)"
    or "<CR>"
end

local set_mappings = function()
  set_keymap("i", "<Tab>", i_Tab, { expr = true, remap = true })
  set_keymap("i", "<S-Tab>", i_STab, { expr = true, remap = true })
  set_keymap("i", "<CR>", i_CR, { expr = true, remap = true })
end

return {
  ping = ping,
  set_mappings = set_mappings,
}
