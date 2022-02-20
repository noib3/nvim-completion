local request = vim.rpcrequest

local ping = function()
  print(request(_G.compleet.channel_id, 'ping', 'Neovim says ping!'))
end

return {
  ping = ping,
}
