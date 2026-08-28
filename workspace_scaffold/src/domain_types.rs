#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]
#[path = "naming_capitalized_parts.rs"]
mod naming_capitalized_parts;
#[path = "naming_kebab_case.rs"]
pub(crate) mod naming_kebab_case;
#[path = "naming_title_case.rs"]
pub(crate) mod naming_title_case;
#[path = "naming_upper_camel_case.rs"]
pub(crate) mod naming_upper_camel_case;
#[path = "naming_validate_project_name.rs"]
pub(crate) mod naming_validate_project_name;
#[path = "naming_validate_repository_url.rs"]
pub(crate) mod naming_validate_repository_url;
#[path = "service_catalog_parse.rs"]
mod service_catalog_parse;
#[path = "service_catalog_render_ci_matrix.rs"]
mod service_catalog_render_ci_matrix;
#[path = "service_catalog_render_release_entries.rs"]
mod service_catalog_render_release_entries;
#[path = "service_catalog_render_release_matrix.rs"]
mod service_catalog_render_release_matrix;
#[path = "service_catalog_string_value.rs"]
mod service_catalog_string_value;

#[path = "project_name_ref.rs"]
mod project_name_ref;
pub(crate) use project_name_ref::*;
#[path = "repository_url_ref.rs"]
mod repository_url_ref;
pub(crate) use repository_url_ref::*;
#[path = "service_port.rs"]
mod service_port;
pub(crate) use service_port::*;
#[path = "scaffold_run_ok.rs"]
mod scaffold_run_ok;
pub(crate) use scaffold_run_ok::*;
#[path = "should_write.rs"]
mod should_write;
pub(crate) use should_write::*;
#[path = "scaffold_text.rs"]
mod scaffold_text;
pub(crate) use scaffold_text::*;
#[path = "scaffold_text_ref.rs"]
mod scaffold_text_ref;
pub(crate) use scaffold_text_ref::*;
#[path = "scaffold_path_ref.rs"]
mod scaffold_path_ref;
pub(crate) use scaffold_path_ref::*;
#[path = "replacements_ref.rs"]
mod replacements_ref;
pub(crate) use replacements_ref::*;
#[path = "cargo_args_ref.rs"]
mod cargo_args_ref;
pub(crate) use cargo_args_ref::*;
#[path = "update_env_name.rs"]
mod update_env_name;
pub(crate) use update_env_name::*;
#[path = "generated_projection.rs"]
mod generated_projection;
pub(crate) use generated_projection::*;
#[path = "should_skip.rs"]
mod should_skip;
pub(crate) use should_skip::*;
#[path = "scaffold_io_error.rs"]
mod scaffold_io_error;
pub(crate) use scaffold_io_error::*;
#[path = "server_runtime_bounded_read_error.rs"]
mod server_runtime_bounded_read_error;
pub(crate) use server_runtime_bounded_read_error::*;
#[path = "scaffold_error.rs"]
mod scaffold_error;
pub(crate) use scaffold_error::*;
#[path = "synchronize_deployment_projections.rs"]
mod synchronize_deployment_projections;
pub(crate) use synchronize_deployment_projections::*;
#[path = "synchronize_cargo_owned_projection.rs"]
mod synchronize_cargo_owned_projection;
pub(crate) use synchronize_cargo_owned_projection::*;
#[path = "scaffold_service.rs"]
mod scaffold_service;
pub(crate) use scaffold_service::*;
#[path = "service_crate.rs"]
mod service_crate;
use service_crate::ServiceCrate;
#[path = "service_compose_name.rs"]
mod service_compose_name;
use service_compose_name::ServiceComposeName;
#[path = "service_compose_file.rs"]
mod service_compose_file;
use service_compose_file::ServiceComposeFile;
#[path = "service_dockerfile.rs"]
mod service_dockerfile;
use service_dockerfile::ServiceDockerfile;
#[path = "service_image.rs"]
mod service_image;
use service_image::ServiceImage;
#[path = "service_kubernetes_manifest.rs"]
mod service_kubernetes_manifest;
use service_kubernetes_manifest::ServiceKubernetesManifest;
#[path = "service_socket_env.rs"]
mod service_socket_env;
use service_socket_env::ServiceSocketEnv;
#[path = "service_catalog_entries.rs"]
mod service_catalog_entries;
use service_catalog_entries::ServiceCatalogEntries;
#[path = "service_catalog_entries_ref.rs"]
mod service_catalog_entries_ref;
use service_catalog_entries_ref::ServiceCatalogEntriesRef;
#[path = "service_catalog_entry.rs"]
mod service_catalog_entry;
use service_catalog_entry::ServiceCatalogEntry;
#[path = "should_release.rs"]
mod should_release;
use should_release::ShouldRelease;
#[path = "service_catalog_draft.rs"]
mod service_catalog_draft;
use service_catalog_draft::ServiceCatalogDraft;
#[path = "synchronize_generated_file.rs"]
mod synchronize_generated_file;
use synchronize_generated_file::synchronize_generated_file;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
