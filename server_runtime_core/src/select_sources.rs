pub const fn select_sources<'source_lt, LocalSource, RemoteSource>(
    optional_local: Option<&'source_lt LocalSource>,
    optional_remote: Option<&'source_lt RemoteSource>,
) -> Result<
    super::SourceSelection<'source_lt, LocalSource, RemoteSource>,
    super::SourceSelectionError,
> {
    match (optional_local, optional_remote) {
        (Some(local), Some(remote)) => Ok(super::SourceSelection::LocalAndRemote { local, remote }),
        (Some(local), None) => Ok(super::SourceSelection::Local(local)),
        (None, Some(remote)) => Ok(super::SourceSelection::Remote(remote)),
        (None, None) => Err(super::SourceSelectionError),
    }
}
