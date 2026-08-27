#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "each bounded type keeps its inherent and trait implementations adjacent"
)]
#[path = "bounded_len.rs"]
mod bounded_len;
#[path = "bounded_string.rs"]
pub mod bounded_string;
#[path = "bounded_value_error.rs"]
mod bounded_value_error;
#[path = "btree.rs"]
pub mod btree;
#[path = "collection_max_len.rs"]
mod collection_max_len;
#[path = "deserialize_bounded_map.rs"]
mod deserialize_bounded_map;
#[path = "hash.rs"]
pub mod hash;
#[path = "serde_prealloc_max_items.rs"]
mod serde_prealloc_max_items;
#[path = "validate_len.rs"]
mod validate_len;
#[path = "vector.rs"]
pub mod vector;

pub use bounded_len::BoundedLen;
pub use bounded_value_error::BoundedValueError;
pub use collection_max_len::COLLECTION_MAX_LEN;
use deserialize_bounded_map::deserialize_bounded_map;
use serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS;
use validate_len::validate_len;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
