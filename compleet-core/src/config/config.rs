use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(default)]
    ui: super::UiConfig,

    #[serde(default)]
    completion: super::CompletionConfig,
}

impl Config {
    /// Whether completion hints are enabled.
    pub const fn hints_enabled(&self) -> bool {
        self.ui.hint.enable
    }
}

/*
```lua
require("compleet").setup({
  ..
  sources = {
    lipsum = { enable = true },

    lsp = {
      enable = function(buf) return true end,
      highlight_completions = true,
    },
  },
})
```

```rust
use compleet_core as compleet;

struct CompleetLsp {
    highlight_completions: bool,
}

#[async_trait]
impl compleet::CompletionSource for CompleetLsp {
    fn name() -> &'static str {
        "lsp"
    }

    async fn complete(&self) -> Vec<String> {
        self.highlight_completions.then(|| vec!["hi".into()])
            .unwrap_or_default()
    }
}
```

```lua
require("compleet").setup({
  ui = {
    menu = {
      anchor = "cursor",
      max_height = 7,
      border = {
        enable = false,
        style = "single",
      }
    },

    -- details = {
    --   border = {
    --     enable = true,
    --     style = "single",
    --   },
    -- },

    hint = { enable = true },
  },

  completion = {
    while_deleting = true,
    after_inserting = false,
  },

  sources = {
    -- lipsum = { enable = true },
    lsp = {
      enable = true,
      highlight_completions = true,
    },
  }
})
```
*/
