#![allow(clippy::wildcard_imports)] // split test fixtures share the private facade vocabulary

#[path = "create_location_test_text.rs"]
mod create_location_test_text;
#[path = "display_struct.rs"]
mod display_struct;
#[path = "error_one.rs"]
mod error_one;
#[path = "error_two.rs"]
mod error_two;
#[path = "error_unnamed_one.rs"]
mod error_unnamed_one;
#[path = "loc_test_text_max_len.rs"]
mod loc_test_text_max_len;
#[path = "location_test_count.rs"]
mod location_test_count;
#[path = "location_test_flag.rs"]
mod location_test_flag;
#[path = "location_test_text.rs"]
mod location_test_text;
#[path = "run.rs"]
mod run;
#[path = "serde_struct.rs"]
mod serde_struct;

use create_location_test_text::create_location_test_text;
pub use display_struct::DisplayStruct;
pub use error_one::*;
pub use error_two::*;
pub use error_unnamed_one::*;
use loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN;
pub use location_test_count::LocationTestCount;
pub use location_test_flag::LocationTestFlag;
pub use location_test_text::*;
pub(crate) use run::run;
pub use serde_struct::SerdeStruct;
