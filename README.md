# :zap: nvim-compleet

This plugin is still in early development.

![preview](./.github/assets/lipsum.gif)

## :book: Table of Contents

- [Installation](#package-installation)
- [Configuration](#wrench-configuration)
- [Sources](#blossom-sources)
- [Commands](#heavy_exclamation_mark-commands)
- [Mappings](#musical_keyboard-mappings)
- [Colors](#art-colors)

## :package: Installation

`nvim-compleet` requires Neovim 0.7+. Also, since the Rust code has to be
compiled it needs the `rustup` toolchain to be available (specifically
`cargo`), together with the `make` and `ar` utilities.

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
          {"",  "CompleetDetails"},
          {"",  "CompleetDetails"},
          {"",  "CompleetDetails"},
          {" ", "CompleetDetails"},
          {"",  "CompleetDetails"},
          {"",  "CompleetDetails"},
          {"",  "CompleetDetails"},
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
    -- Completes words from a Lorem Ipsum paragraph. Mostly used for testing.
    lipsum = {
      enable = false,
    },
  }
})
```

## :blossom: Sources

The only completion source currently available is `lipsum`, which completes
words from a [Lorem ipsum](https://en.wikipedia.org/wiki/Lorem_ipsum)
paragraph.

Actually useful sources are yet to be developed.

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

## :art: Colors

The following highlight groups can be used to theme `nvim-compleet`'s UI:

  * `CompleetMenu`: used to highlight the completion menu. Linked to
    `NormalFloat` by default;

  * `CompleetMenuSelected`: used to highlight the currently selected completion
    item. Linked to `PmenuSel` by default;

  * `CompleetMenuMatchingChars`: used to highlight the characters where a
    completion item matches the current completion prefix. Linked to
    `Statement` by default;

  * `CompleetMenuBorder`: used to highlight the border of the completion menu.
    Linked to `FloatBorder` by default;

  * `CompleetDetails`: used to highlight the details window. Linked to
    `NormalFloat` by default;

  * `CompleetDetailsBorder`:  used to highlight the border of the details
    window. Linked to `FloatBorder` by default;

  * `CompleetHint`: used to highlight the completion hint. Linked to `Comment`
    by default.
