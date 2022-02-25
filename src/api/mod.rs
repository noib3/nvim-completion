mod accept_completion;
mod cursor_moved;
mod has_completions;
mod insert_left;
mod is_completion_item_selected;
mod is_completion_menu_visible;
mod select_next_completion;
mod select_prev_completion;
mod setup;
mod show_completions;
mod text_changed;

use accept_completion::accept_completion;

// Executed on every `CursorMovedI` event.
pub use cursor_moved::cursor_moved;

// Exposed to users.
pub use has_completions::has_completions;

// Executed on every `InsertLeft` event.
pub use insert_left::insert_left;

// Exposed to users.
pub use is_completion_item_selected::is_completion_item_selected;

// Exposed to users.
pub use is_completion_menu_visible::is_completion_menu_visible;

use select_next_completion::select_next_completion;

use select_prev_completion::select_prev_completion;

// Exposed to users.
pub use setup::setup;

use show_completions::show_completions;

// Executed on every `TextChangedI` event.
pub use text_changed::text_changed;
