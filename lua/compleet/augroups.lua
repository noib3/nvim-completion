local setup = function()
  vim.cmd([[
    augroup compleet_notify_events
      autocmd CursorMovedI * lua require('compleet').notify('CursorMovedI')
      autocmd InsertLeave * lua require('compleet').notify('InsertLeave')
      autocmd InsertCharPre *
        \ execute printf(
        \   'lua require("compleet").notify("InsertCharPre", "%s")',
        \   v:char
        \ )
    augroup END
  ]])
end

return {
  setup = setup,
}
