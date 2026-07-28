#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceSelection<'source_lt, LocalSource, RemoteSource> {
    Local(&'source_lt LocalSource),
    LocalAndRemote {
        local: &'source_lt LocalSource,
        remote: &'source_lt RemoteSource,
    },
    Remote(&'source_lt RemoteSource),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", str_constants::SOURCE_SELECTION_REQUIRES_AT_LEAST_ONE_SOURCE)]
pub struct SourceSelectionError;

pub const fn select_sources<'source_lt, LocalSource, RemoteSource>(
    optional_local: Option<&'source_lt LocalSource>,
    optional_remote: Option<&'source_lt RemoteSource>,
) -> Result<SourceSelection<'source_lt, LocalSource, RemoteSource>, SourceSelectionError> {
    match (optional_local, optional_remote) {
        (Some(local), Some(remote)) => Ok(SourceSelection::LocalAndRemote { local, remote }),
        (Some(local), None) => Ok(SourceSelection::Local(local)),
        (None, Some(remote)) => Ok(SourceSelection::Remote(remote)),
        (None, None) => Err(SourceSelectionError),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn selection_preserves_every_valid_source_combination() {
        let local = 1u8;
        let remote = 2u8;
        assert_eq!(
            super::select_sources(Some(&local), None::<&u8>),
            Ok(super::SourceSelection::Local(&local))
        );
        assert_eq!(
            super::select_sources(None::<&u8>, Some(&remote)),
            Ok(super::SourceSelection::Remote(&remote))
        );
        assert_eq!(
            super::select_sources(Some(&local), Some(&remote)),
            Ok(super::SourceSelection::LocalAndRemote {
                local: &local,
                remote: &remote,
            })
        );
    }

    #[test]
    fn selection_rejects_missing_sources() {
        assert_eq!(
            super::select_sources(None::<&u8>, None::<&u8>),
            Err(super::SourceSelectionError)
        );
    }
}
