#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub struct DuplicateCandidates<T>(Vec<T>);
