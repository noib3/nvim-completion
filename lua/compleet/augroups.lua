local setup = function()
  vim.cmd([[
    augroup compleet_notify_events
      autocmd CursorMovedI * lua require('compleet').cursor_moved()
      autocmd InsertLeave * lua require('compleet').insert_left()
      autocmd TextChangedI * lua require('compleet').text_changed()
    augroup END
  ]])
end

return {
  setup = setup,
}
