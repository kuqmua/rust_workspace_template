#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::module_inception,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[path = "admin_fixture_string/admin_fixture_string.rs"]
mod admin_fixture_string;

pub(crate) use admin_fixture_string::admin_fixture_string;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::BoundedString)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(crate) struct AdminFixtureString(pub(super) String);
