mod vec_display;
mod test_context;
mod cmd_utils;

pub use vec_display::VecDisplay;
pub use test_context::{setup, TestContext};
pub use cmd_utils::{read_until_question_mark};

pub const BINARY_NAME: &str = "bs";


