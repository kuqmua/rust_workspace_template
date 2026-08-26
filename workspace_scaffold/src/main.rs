mod adapters;
mod domain_types;

fn workspace_root() -> Result<domain_types::ScaffoldPathRef<'static>, domain_types::ScaffoldError> {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(domain_types::ScaffoldPathRef::from)
        .ok_or(domain_types::ScaffoldError::Arguments)
}

fn main() {
    let run_ok = {
        let result = (|| {
            let mut arguments = std::env::args().skip(constants_usize::ONE);
            match arguments.next().as_deref() {
                Some(constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND) => {
                    let name = arguments
                        .next()
                        .ok_or(domain_types::ScaffoldError::Arguments)?;
                    let repository_url = arguments
                        .next()
                        .ok_or(domain_types::ScaffoldError::Arguments)?;
                    if arguments.next().is_some() {
                        return Err(domain_types::ScaffoldError::Arguments);
                    }
                    let name_ref = domain_types::ProjectNameRef::from(name.as_str());
                    let repository_url_ref =
                        domain_types::RepositoryUrlRef::from(repository_url.as_str());
                    domain_types::naming_validate_project_name::naming_validate_project_name(
                        name_ref,
                    )?;
                    domain_types::naming_validate_repository_url::naming_validate_repository_url(
                        repository_url_ref,
                    )?;
                    adapters::template_fs_rename_identity::template_fs_rename_identity(
                        workspace_root()?,
                        name_ref,
                        repository_url_ref,
                    )
                }
                Some(constants_str::SERVICE) => {
                    let name = arguments
                        .next()
                        .ok_or(domain_types::ScaffoldError::Arguments)?;
                    let port = match arguments
                        .next()
                        .ok_or(domain_types::ScaffoldError::Arguments)?
                        .parse::<u16>()
                    {
                        Ok(value) => domain_types::ServicePort::from(value),
                        Err(_error) => return Err(domain_types::ScaffoldError::ServicePort),
                    };
                    if arguments.next().is_some() {
                        return Err(domain_types::ScaffoldError::Arguments);
                    }
                    domain_types::scaffold_service(
                        workspace_root()?,
                        domain_types::ProjectNameRef::from(name.as_str()),
                        port,
                    )
                }
                Some(constants_str::VALUE_24CACF50) => {
                    let write_changes = match arguments.next().as_deref() {
                        Some(constants_str::SYNC) => domain_types::ShouldWrite::from(true),
                        Some(constants_str::CHECK) => domain_types::ShouldWrite::from(false),
                        Some(_) | None => {
                            return Err(domain_types::ScaffoldError::Arguments);
                        }
                    };
                    if arguments.next().is_some() {
                        return Err(domain_types::ScaffoldError::Arguments);
                    }
                    let root = workspace_root()?;
                    domain_types::synchronize_deployment_projections(root, write_changes)?;
                    domain_types::synchronize_cargo_owned_projection(
                        root,
                        domain_types::CargoArgsRef::from(
                            &[
                                constants_str::TEST_ALT_3,
                                constants_str::P,
                                constants_str::VALUE_B2F5A0ED,
                                constants_str::P,
                                constants_str::VALUE_8B9F9090,
                                constants_str::VALUE_B43DA2C2,
                            ][..],
                        ),
                        domain_types::UpdateEnvName::from(constants_str::UPDATE_CONFIG_PROJECTIONS),
                        domain_types::GeneratedProjection::Config,
                        write_changes,
                    )?;
                    domain_types::synchronize_cargo_owned_projection(
                        root,
                        domain_types::CargoArgsRef::from(
                            &[
                                constants_str::TEST_ALT_3,
                                constants_str::P,
                                constants_str::TESTS_ALT,
                                constants_str::CODE_STYLE,
                            ][..],
                        ),
                        domain_types::UpdateEnvName::from(
                            constants_str::UPDATE_CODE_STYLE_SNAPSHOTS,
                        ),
                        domain_types::GeneratedProjection::CodeStyle,
                        write_changes,
                    )
                }
                Some(constants_str::VALUE_AEE50B18) => {
                    let write_changes = match arguments.next().as_deref() {
                        Some(constants_str::SYNC) => domain_types::ShouldWrite::from(true),
                        Some(constants_str::CHECK) => domain_types::ShouldWrite::from(false),
                        Some(_) | None => {
                            return Err(domain_types::ScaffoldError::Arguments);
                        }
                    };
                    if arguments.next().is_some() {
                        return Err(domain_types::ScaffoldError::Arguments);
                    }
                    domain_types::synchronize_deployment_projections(
                        workspace_root()?,
                        write_changes,
                    )
                }
                Some(_) | None => Err(domain_types::ScaffoldError::Arguments),
            }
        })();
        match result {
            Ok(()) => domain_types::ScaffoldRunOk::from(true),
            Err(error) => {
                tracing::error!(error = %error, "workspace scaffolding failed");
                domain_types::ScaffoldRunOk::from(false)
            }
        }
    };
    if !run_ok.get() {
        std::process::exit(2i32);
    }
}
