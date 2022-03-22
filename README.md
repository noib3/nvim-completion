# :zap: nvim-compleet

This plugin is still in early development.

## :book: Table of Contents

- [Installation](#package-installation)
- [Configuration](#wrench-configuration)
- [Commands](#heavy_exclamation_mark-commands)
- [Mappings](#musical_keyboard-mappings)
- [Colors](#art-colors)

## :package: Installation

Requires Neovim 0.7+.

TODO

## :wrench: Configuration

TODO

## :heavy_exclamation_mark: Commands

TODO

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
