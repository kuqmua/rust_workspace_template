#![cfg_attr(test, allow(unused_crate_dependencies))] // tower is used by the separate admin_api integration test target

#[path = "domain_types.rs"]
pub mod domain_types;
#[path = "migrations.rs"]
mod migrations;
#[path = "repository.rs"]
mod repository;
