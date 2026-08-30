#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceSelection<'source_lt, LocalSource, RemoteSource> {
    Local(&'source_lt LocalSource),
    LocalAndRemote {
        local: &'source_lt LocalSource,
        remote: &'source_lt RemoteSource,
    },
    Remote(&'source_lt RemoteSource),
}
#[cfg(test)]
mod tests {
    #[test]
    fn selection_preserves_every_valid_source_combination() {
        let local = 1u8;
        let remote = 2u8;
        assert_eq!(
            crate::select_sources::select_sources(Some(&local), None::<&u8>),
            Ok(super::SourceSelection::Local(&local))
        );
        assert_eq!(
            crate::select_sources::select_sources(None::<&u8>, Some(&remote)),
            Ok(super::SourceSelection::Remote(&remote))
        );
        assert_eq!(
            crate::select_sources::select_sources(Some(&local), Some(&remote)),
            Ok(super::SourceSelection::LocalAndRemote {
                local: &local,
                remote: &remote,
            })
        );
    }

    #[test]
    fn selection_rejects_missing_sources() {
        assert_eq!(
            crate::select_sources::select_sources(None::<&u8>, None::<&u8>),
            Err(crate::source_selection_error::SourceSelectionError::Missing)
        );
    }
}
