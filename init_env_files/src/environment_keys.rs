pub(super) fn environment_keys(
    content: crate::EnvContentRef<'_>,
) -> Result<crate::EnvKeys, crate::InitStringError> {
    content
        .as_ref()
        .lines()
        .filter_map(|source_line| {
            let trimmed_line = source_line.trim();
            (!trimmed_line.is_empty() && !trimmed_line.starts_with('#'))
                .then(|| {
                    trimmed_line
                        .split_once('=')
                        .map(|(key, _value)| crate::EnvKey::try_from(key.trim().to_owned()))
                })
                .flatten()
        })
        .collect::<Result<Vec<crate::EnvKey>, crate::InitStringError>>()
        .map(bounded_types::BoundedVec::from_max_iter)
        .map(crate::EnvKeys::from)
}
