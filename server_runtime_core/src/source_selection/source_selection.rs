#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceSelection<'source_lt, LocalSource, RemoteSource> {
    Local(&'source_lt LocalSource),
    LocalAndRemote {
        local: &'source_lt LocalSource,
        remote: &'source_lt RemoteSource,
    },
    Remote(&'source_lt RemoteSource),
}
