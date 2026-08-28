use crate::domain_types::{EnvContentRef, EnvKey, EnvKeys, InitStringError};

pub(super) fn environment_keys(content: EnvContentRef<'_>) -> Result<EnvKeys, InitStringError> {
    content
        .as_ref()
        .lines()
        .filter_map(|source_line| {
            let trimmed_line = source_line.trim();
            (!trimmed_line.is_empty() && !trimmed_line.starts_with('#'))
                .then(|| {
                    trimmed_line
                        .split_once('=')
                        .map(|(key, _value)| EnvKey::try_from(key.trim().to_owned()))
                })
                .flatten()
        })
        .collect::<Result<Vec<EnvKey>, InitStringError>>()
        .map(bounded_types::domain_types::vector::BoundedVec::from_max_iter)
        .map(EnvKeys::from)
}
