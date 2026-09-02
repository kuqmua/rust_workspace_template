pub(super) fn environment_keys(
    env_content_ref: crate::env_content_ref::EnvContentRef<'_>,
) -> Result<crate::env_keys::EnvKeys, crate::init_string_error::InitStringError> {
    env_content_ref
        .as_ref()
        .lines()
        .filter_map(|source_line| {
            let trimmed_line = source_line.trim();
            (!trimmed_line.is_empty() && !trimmed_line.starts_with('#'))
                .then(|| {
                    trimmed_line.split_once('=').map(|(key, _value)| {
                        crate::env_key::EnvKey::try_from(key.trim().to_owned())
                    })
                })
                .flatten()
        })
        .collect::<Result<Vec<crate::env_key::EnvKey>, crate::init_string_error::InitStringError>>()
        .map(bounded_types::bounded_vec::BoundedVec::from_max_iter)
        .map(crate::env_keys::EnvKeys::from)
}
