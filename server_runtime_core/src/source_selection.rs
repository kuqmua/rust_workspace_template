#[path = "source_selection/select_sources.rs"]
mod select_sources;
#[path = "source_selection/source_selection.rs"]
mod source_selection;
#[path = "source_selection/source_selection_error.rs"]
mod source_selection_error;

pub use select_sources::select_sources;
pub use source_selection::SourceSelection;
pub use source_selection_error::SourceSelectionError;

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
