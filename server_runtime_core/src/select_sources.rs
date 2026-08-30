pub const fn select_sources<'source_lt, LocalSource, RemoteSource>(
    optional_local: Option<&'source_lt LocalSource>,
    optional_remote: Option<&'source_lt RemoteSource>,
) -> Result<
    crate::source_selection::SourceSelection<'source_lt, LocalSource, RemoteSource>,
    crate::source_selection_error::SourceSelectionError,
> {
    match (optional_local, optional_remote) {
        (Some(local), Some(remote)) => {
            Ok(crate::source_selection::SourceSelection::LocalAndRemote { local, remote })
        }
        (Some(local), None) => Ok(crate::source_selection::SourceSelection::Local(local)),
        (None, Some(remote)) => Ok(crate::source_selection::SourceSelection::Remote(remote)),
        (None, None) => Err(crate::source_selection_error::SourceSelectionError::Missing),
    }
}
