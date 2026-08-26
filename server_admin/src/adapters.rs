#![allow(clippy::arbitrary_source_item_ordering)] // adapter declarations are grouped by repository transaction responsibility

#[path = "migrations.rs"]
pub(crate) mod migrations;
#[path = "repository.rs"]
pub(crate) mod repository;
