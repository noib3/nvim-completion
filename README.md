# :zap: nvim-compleet

This plugin is still in early development.

![preview](https://user-images.githubusercontent.com/2746374/160488112-4686a0f2-92ac-4bbb-974b-a8e86e27c042.gif)

## :book: Table of Contents

- [Installation](#package-installation)
- [Features](#rocket-features)
- [Configuration](#wrench-configuration)
- [Sources](https://github.com/noib3/nvim-compleet/wiki/Sources)
- [Commands](#heavy_exclamation_mark-commands)
- [Mappings](#musical_keyboard-mappings)
- [Colors](https://github.com/noib3/nvim-compleet/wiki/Highlight-groups)
- [Roadmap](#chart_with_upwards_trend-roadmap)

## :package: Installation

`nvim-compleet` requires Neovim 0.7+. Also, since the Rust code has to be
compiled it needs the `rustup` toolchain to be available (follow [this
guide](https://www.rust-lang.org/tools/install) for instructions on how to
install Rust) with `rustc` version 1.58+, together with the `make` and `ar`
utilities.

Then installing the plugin is as easy as

```lua
require("packer").startup(function()
  use({
    "noib3/nvim-compleet",
    config = function()
      require("compleet").setup()
    end,
    run = "cargo build --release && make install",
  })
end)
```

## :rocket: Features

### Config validation

![validation1](https://user-images.githubusercontent.com/2746374/160488191-13cb17f2-de50-49f3-a97a-adc595d54af9.png)
![validation2](https://user-images.githubusercontent.com/2746374/160488196-e628541a-d615-47bc-8e33-c43102af12eb.png)
![validation3](https://user-images.githubusercontent.com/2746374/160488210-7f0c0946-f814-4553-9a2d-ede74e969042.png)

## :wrench: Configuration

`nvim-compleet` is configured by passing a table to the `setup` function. The
default config is

```lua
require('compleet').setup({
  ui = {
    menu = {
      -- Where to anchor the completion menu, either "cursor" or "match".
      anchor = "cursor",

      -- Whether to automatically show the menu every time there are
      -- completions available.
      autoshow = true,

      -- The maximum height (in rows) of the completion menu.
      max_height = nil,

      border = {
        -- Whether to add a border to the completion menu's floating window.
        enable = false,

        -- Any of the style formats listed in `:h nvim_open_win`.
        style = "single"
      },
    },

    details = {
      border = {
        -- Whether to add a border to the details's floating window.
        enable = true,

        -- Same as `ui.menu.border.style`.
        style = {
          "",
          "",
          "",
          {" ", "CompleetDetails"},
        }
      },
    },

    hint = {
      -- Whether to show completion hints.
      enable = false,
    }
  },

  completion = {
    -- Whether to enable completion while deleting characters.
    while_deleting = false,
  },

  sources = {
    lipsum = {
      enable = false,
    },
  }
})
```

## :heavy_exclamation_mark: Commands

`nvim-compleet` provides two commands: `CompleetStop{!}` to stop the completion
and `CompleetStart{!}` to restart it. The versions with the bang `!` stop/start
the completion in all the buffers, the ones without it only affect the current
buffer.

## :musical_keyboard: Mappings

The following key mappings are exposed:

  * `<Plug>(compleet-next-completion)`: selects the next item in the completion
    menu;

  * `<Plug>(compleet-prev-completion)`: selects the previous item in the
    completion menu;

  * `<Plug>(compleet-insert-selected-completion)`: inserts the currently
    selected completion item into the buffer;

  * `<Plug>(compleet-insert-hinted-completion)`: inserts the currently
    hinted completion item into the buffer. Useful when hints are enabled and
    `ui.menu.autoshow` is set to `false`;

  * `<Plug>(compleet-show-completions)`: shows all the available completions at
    the current cursor position.

A possible configuration could be:

```lua
local compleet = require('compleet')
local keymap = vim.keymap

local tab = function()
  return
    (compleet.is_menu_visible() and "<Plug>(compleet-next-completion)")
    or (compleet.has_completions() and "<Plug>(compleet-show-completions)")
    or "<Tab>"
end

local s_tab = function()
  return
    compleet.is_menu_visible()
    and "<Plug>(compleet-prev-completion)"
     or "<S-Tab>"
end

local right = function()
  return
    compleet.is_hint_visible()
    and "<Plug>(compleet-insert-hinted-completion)"
     or "<Right>"
end

local cr = function()
  return
    compleet.is_completion_selected()
    and "<Plug>(compleet-insert-selected-completion)"
     or "<CR>"
end

local opts = { expr = true, remap = true }

keymap.set("i", "<Tab>", tab, opts)
keymap.set("i", "<S-Tab>", s_tab, opts)
keymap.set("i", "<Right>", right, opts)
keymap.set("i", "<CR>", cr, opts)
```

## :chart_with_upwards_trend: Roadmap

- [ ] Add LSP source;
- [ ] Add Filepath source;
- [ ] Add Treesitter source;
- [ ] Integrate with snippets engines;
- [ ] Stabilize api, document how to add sources in Rust, add option to provide
  user-defined sources in Lua;
- [ ] ...
