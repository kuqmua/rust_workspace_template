#[must_use]
pub fn classify_not_found_io_error(
    error: super::BoundedReadIoError,
) -> super::IoErrorPresenceDisposition {
    if error.0.kind() == std::io::ErrorKind::NotFound {
        super::IoErrorPresenceDisposition::Missing
    } else {
        super::IoErrorPresenceDisposition::Other(error)
    }
}
