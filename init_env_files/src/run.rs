#![allow(clippy::single_call_fn)] // the binary entrypoint has one application composition owner

pub(crate) fn run() -> Result<(), crate::domain_types::InitializeError> {
    let mode = if std::env::args().any(|argument| argument == constants_str::DRY_RUN) {
        crate::domain_types::RunMode::DryRun
    } else {
        crate::domain_types::RunMode::Apply
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(crate::domain_types::InitializeError::MembersMissing)?;
    crate::domain_types::initialize(crate::domain_types::WorkspaceRootPathRef::from(root), mode)?
        .into_iter()
        .for_each(|entry| {
            let separator = constants_str::COMMA_SPACE.trim();
            let keys_capacity = entry
                .keys()
                .as_ref()
                .iter()
                .map(|key| key.as_ref().len())
                .sum::<usize>()
                .saturating_add(
                    entry
                        .keys()
                        .as_ref()
                        .len()
                        .get()
                        .saturating_sub(constants_usize::ONE)
                        .saturating_mul(separator.len()),
                );
            let keys = entry.keys().as_ref().iter().enumerate().fold(
                String::with_capacity(keys_capacity),
                |mut keys, (index, key)| {
                    if index > constants_usize::ZERO {
                        keys.push_str(separator);
                    }
                    keys.push_str(key.as_ref());
                    keys
                },
            );
            tracing::info!(
                member = entry.member().as_ref(),
                status = ?entry.status(),
                keys,
                "environment file initialization completed"
            );
        });
    Ok(())
}
