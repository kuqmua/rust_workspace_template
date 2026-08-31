#[must_use]
pub fn classify_not_found_io_error(
    error: crate::bounded_read_io_error::BoundedReadIoError,
) -> crate::io_error_presence_disposition::IoErrorPresenceDisposition {
    if error.kind() == std::io::ErrorKind::NotFound {
        crate::io_error_presence_disposition::IoErrorPresenceDisposition::Missing
    } else {
        crate::io_error_presence_disposition::IoErrorPresenceDisposition::Other(error)
    }
}
