fn workspace_root()
-> Result<crate::domain_types::ScaffoldPathRef<'static>, crate::domain_types::ScaffoldError> {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(crate::domain_types::ScaffoldPathRef::from)
        .ok_or(crate::domain_types::ScaffoldError::Arguments)
}

#[allow(
    clippy::single_call_fn,
    reason = "binary entry point delegates fallible argument handling"
)]
fn run() -> Result<(), crate::domain_types::ScaffoldError> {
    let mut arguments = std::env::args().skip(constants_usize::ONE);
    match arguments.next().as_deref() {
        Some(constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND) => {
            let name = arguments
                .next()
                .ok_or(crate::domain_types::ScaffoldError::Arguments)?;
            let repository_url = arguments
                .next()
                .ok_or(crate::domain_types::ScaffoldError::Arguments)?;
            if arguments.next().is_some() {
                return Err(crate::domain_types::ScaffoldError::Arguments);
            }
            let name_ref = crate::domain_types::ProjectNameRef::from(name.as_str());
            let repository_url_ref =
                crate::domain_types::RepositoryUrlRef::from(repository_url.as_str());
            crate::domain_types::naming::validate_project_name(name_ref)?;
            crate::domain_types::naming::validate_repository_url(repository_url_ref)?;
            crate::adapters::template_fs::rename_identity(
                workspace_root()?,
                name_ref,
                repository_url_ref,
            )
        }
        Some(constants_str::SERVICE) => {
            let name = arguments
                .next()
                .ok_or(crate::domain_types::ScaffoldError::Arguments)?;
            let port = match arguments
                .next()
                .ok_or(crate::domain_types::ScaffoldError::Arguments)?
                .parse::<u16>()
            {
                Ok(value) => crate::domain_types::ServicePort::from(value),
                Err(_error) => return Err(crate::domain_types::ScaffoldError::ServicePort),
            };
            if arguments.next().is_some() {
                return Err(crate::domain_types::ScaffoldError::Arguments);
            }
            crate::domain_types::scaffold_service(
                workspace_root()?,
                crate::domain_types::ProjectNameRef::from(name.as_str()),
                port,
            )
        }
        Some(constants_str::VALUE_24CACF50) => {
            let write_changes = match arguments.next().as_deref() {
                Some(constants_str::SYNC) => crate::domain_types::ShouldWrite::from(true),
                Some(constants_str::CHECK) => crate::domain_types::ShouldWrite::from(false),
                Some(_) | None => return Err(crate::domain_types::ScaffoldError::Arguments),
            };
            if arguments.next().is_some() {
                return Err(crate::domain_types::ScaffoldError::Arguments);
            }
            crate::domain_types::synchronize_all_generated_artifacts(
                workspace_root()?,
                write_changes,
            )
        }
        Some(constants_str::VALUE_AEE50B18) => {
            let write_changes = match arguments.next().as_deref() {
                Some(constants_str::SYNC) => crate::domain_types::ShouldWrite::from(true),
                Some(constants_str::CHECK) => crate::domain_types::ShouldWrite::from(false),
                Some(_) | None => return Err(crate::domain_types::ScaffoldError::Arguments),
            };
            if arguments.next().is_some() {
                return Err(crate::domain_types::ScaffoldError::Arguments);
            }
            crate::domain_types::synchronize_deployment_projections(
                workspace_root()?,
                write_changes,
            )
        }
        Some(_) | None => Err(crate::domain_types::ScaffoldError::Arguments),
    }
}

#[allow(
    clippy::single_call_fn,
    reason = "the executable adapter delegates scaffolding and error reporting to its owned module"
)]
pub(crate) fn run_ok() -> crate::domain_types::ScaffoldRunOk {
    match run() {
        Ok(()) => crate::domain_types::ScaffoldRunOk::from(true),
        Err(error) => {
            tracing::error!(error = %error, "workspace scaffolding failed");
            crate::domain_types::ScaffoldRunOk::from(false)
        }
    }
}
