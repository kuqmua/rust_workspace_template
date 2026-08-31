#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "each bounded type keeps its inherent and trait implementations adjacent"
)]

pub mod bounded_b_tree_map;
pub mod bounded_b_tree_map_visitor_phantom_data;
pub mod bounded_chars_string;
pub mod bounded_hash_map;
pub mod bounded_hash_map_visitor_phantom_data;
pub mod bounded_len;
pub mod bounded_string;
pub mod bounded_string_error;
pub mod bounded_value_error;
pub mod bounded_vec;
pub mod bounded_vec_visitor_phantom_data;
pub mod collection_max_len;
pub mod deserialize_bounded_map;
pub(crate) mod deserialize_bounded_owned_string;
pub mod serde_prealloc_max_items;
pub mod validate_len;

#[cfg(test)]
pub mod test_tests;
