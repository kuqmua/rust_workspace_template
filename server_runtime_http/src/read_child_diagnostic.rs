// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(
    clippy::single_call_fn,
    reason = "bounded diagnostic reader remains directly exercised by focused tests"
)]
pub(super) async fn read_child_diagnostic<Reader>(
    mut reader: Reader,
    maximum: crate::child_diagnostic_maximum_non_zero_usize::ChildDiagnosticMaximumNonZeroUsize,
) -> Result<crate::child_diagnostic::ChildDiagnostic, crate::child_process_error::ChildProcessError>
where
    Reader: tokio::io::AsyncRead + Unpin,
{
    let mut output = Vec::with_capacity(maximum.get());
    let mut buffer = [constants_u8::ZERO; constants_usize::VALUE_4_096];
    while output.len() < maximum.get() {
        let remaining = maximum.get().saturating_sub(output.len());
        let read_length = remaining.min(buffer.len());
        let target = buffer
            .get_mut(..read_length)
            .ok_or(crate::child_process_error::ChildProcessError::DiagnosticRange)?;
        let read = tokio::io::AsyncReadExt::read(&mut reader, target)
            .await
            .map_err(crate::child_process_io_error::ChildProcessIoError::from)
            .map_err(crate::child_process_error::ChildProcessError::DiagnosticIo)?;
        if read == constants_usize::ZERO {
            break;
        }
        let read_bytes = buffer
            .get(..read)
            .ok_or(crate::child_process_error::ChildProcessError::DiagnosticRange)?;
        output.extend_from_slice(read_bytes);
    }
    Ok(crate::child_diagnostic::ChildDiagnostic::from(
        bounded_types::bounded_vec::BoundedVec::from_max_iter(output),
    ))
}
