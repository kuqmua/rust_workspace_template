#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "each bounded type keeps its inherent and trait implementations adjacent"
)]

mod bounded_b_tree_map;
mod bounded_b_tree_map_visitor_phantom_data;
mod bounded_hash_map;
mod bounded_hash_map_visitor_phantom_data;
mod bounded_len;
pub mod bounded_string;
mod bounded_value_error;
mod bounded_vec;
mod bounded_vec_visitor_phantom_data;
pub mod btree;
mod collection_max_len;
mod deserialize_bounded_map;
pub mod domain_types;
pub mod hash;
mod serde_prealloc_max_items;
mod validate_len;
pub mod vector;

pub(crate) use bounded_len::BoundedLen;
pub(crate) use bounded_value_error::BoundedValueError;
pub(crate) use deserialize_bounded_map::deserialize_bounded_map;
pub(crate) use serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS;
pub(crate) use validate_len::validate_len;

#[cfg(test)]
mod tests;
