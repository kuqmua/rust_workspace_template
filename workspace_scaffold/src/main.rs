#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "root-owned scaffold stages preserve the former owner-module grouping and shared facade vocabulary"
)]

mod cargo_args_ref;
mod domain_types;
mod generated_projection;
mod naming_capitalized_parts;
mod naming_kebab_case;
mod naming_title_case;
mod naming_upper_camel_case;
mod naming_validate_project_name;
mod naming_validate_repository_url;
mod project_name_ref;
mod replacements_ref;
mod repository_url_ref;
mod scaffold_error;
mod scaffold_io_error;
mod scaffold_path_ref;
mod scaffold_run_ok;
mod scaffold_service;
mod scaffold_text;
mod scaffold_text_ref;
mod server_runtime_bounded_read_error;
mod service_catalog_draft;
mod service_catalog_entries;
mod service_catalog_entries_ref;
mod service_catalog_entry;
mod service_catalog_parse;
mod service_catalog_render_release_entries;
mod service_catalog_string_value;
mod service_compose_file;
mod service_compose_name;
mod service_crate;
mod service_dockerfile;
mod service_image;
mod service_kubernetes_manifest;
mod service_port;
mod service_socket_env;
mod should_release;
mod should_skip;
mod should_write;
mod synchronize_cargo_owned_projection;
mod synchronize_deployment_projections;
mod synchronize_generated_file;
mod template_fs_copy_template_tree;
mod template_fs_insert_once;
mod template_fs_read_bounded_text;
mod template_fs_rename_identity;
mod template_fs_replace_file;
mod template_fs_should_skip;
mod template_fs_write_text;
#[cfg(test)]
mod tests;
mod update_env_name;

pub(crate) use domain_types::*;

fn workspace_root() -> Result<ScaffoldPathRef<'static>, ScaffoldError> {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(ScaffoldPathRef::from)
        .ok_or(ScaffoldError::Arguments)
}

fn main() {
    let run_ok = {
        let result = (|| {
            let mut arguments = std::env::args().skip(constants_usize::ONE);
            match arguments.next().as_deref() {
                Some(constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND) => {
                    let name = arguments.next().ok_or(ScaffoldError::Arguments)?;
                    let repository_url = arguments.next().ok_or(ScaffoldError::Arguments)?;
                    if arguments.next().is_some() {
                        return Err(ScaffoldError::Arguments);
                    }
                    let name_ref = ProjectNameRef::from(name.as_str());
                    let repository_url_ref = RepositoryUrlRef::from(repository_url.as_str());
                    naming_validate_project_name::naming_validate_project_name(name_ref)?;
                    naming_validate_repository_url::naming_validate_repository_url(
                        repository_url_ref,
                    )?;
                    template_fs_rename_identity::template_fs_rename_identity(
                        workspace_root()?,
                        name_ref,
                        repository_url_ref,
                    )
                }
                Some(constants_str::SERVICE) => {
                    let name = arguments.next().ok_or(ScaffoldError::Arguments)?;
                    let port = match arguments
                        .next()
                        .ok_or(ScaffoldError::Arguments)?
                        .parse::<u16>()
                    {
                        Ok(value) => ServicePort::from(value),
                        Err(_error) => return Err(ScaffoldError::ServicePort),
                    };
                    if arguments.next().is_some() {
                        return Err(ScaffoldError::Arguments);
                    }
                    scaffold_service(workspace_root()?, ProjectNameRef::from(name.as_str()), port)
                }
                Some(constants_str::VALUE_24CACF50) => {
                    let write_changes = match arguments.next().as_deref() {
                        Some(constants_str::SYNC) => ShouldWrite::from(true),
                        Some(constants_str::CHECK) => ShouldWrite::from(false),
                        Some(_) | None => {
                            return Err(ScaffoldError::Arguments);
                        }
                    };
                    if arguments.next().is_some() {
                        return Err(ScaffoldError::Arguments);
                    }
                    let root = workspace_root()?;
                    synchronize_deployment_projections(root, write_changes)?;
                    synchronize_cargo_owned_projection(
                        root,
                        CargoArgsRef::from(
                            &[
                                constants_str::TEST_ALT_3,
                                constants_str::P,
                                constants_str::VALUE_B2F5A0ED,
                                constants_str::P,
                                constants_str::VALUE_8B9F9090,
                                constants_str::VALUE_B43DA2C2,
                            ][..],
                        ),
                        UpdateEnvName::from(constants_str::UPDATE_CONFIG_PROJECTIONS),
                        GeneratedProjection::Config,
                        write_changes,
                    )?;
                    synchronize_cargo_owned_projection(
                        root,
                        CargoArgsRef::from(
                            &[
                                constants_str::TEST_ALT_3,
                                constants_str::P,
                                constants_str::TESTS_ALT,
                                constants_str::CODE_STYLE,
                            ][..],
                        ),
                        UpdateEnvName::from(constants_str::UPDATE_CODE_STYLE_SNAPSHOTS),
                        GeneratedProjection::CodeStyle,
                        write_changes,
                    )
                }
                Some(constants_str::VALUE_AEE50B18) => {
                    let write_changes = match arguments.next().as_deref() {
                        Some(constants_str::SYNC) => ShouldWrite::from(true),
                        Some(constants_str::CHECK) => ShouldWrite::from(false),
                        Some(_) | None => {
                            return Err(ScaffoldError::Arguments);
                        }
                    };
                    if arguments.next().is_some() {
                        return Err(ScaffoldError::Arguments);
                    }
                    synchronize_deployment_projections(workspace_root()?, write_changes)
                }
                Some(_) | None => Err(ScaffoldError::Arguments),
            }
        })();
        match result {
            Ok(()) => ScaffoldRunOk::from(true),
            Err(error) => {
                tracing::error!(error = %error, "workspace scaffolding failed");
                ScaffoldRunOk::from(false)
            }
        }
    };
    if !run_ok.get() {
        std::process::exit(2i32);
    }
}
