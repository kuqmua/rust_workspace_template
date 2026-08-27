#[allow(clippy::single_call_fn)]
pub(super) async fn read_child_diagnostic<Reader>(
    mut reader: Reader,
    maximum: super::ChildDiagnosticMaximumNonZeroUsize,
) -> Result<super::ChildDiagnostic, super::ChildProcessError>
where
    Reader: tokio::io::AsyncRead + Unpin,
{
    let mut output = Vec::with_capacity(maximum.0.get());
    let mut buffer = [constants_u8::ZERO; constants_usize::VALUE_4_096];
    while output.len() < maximum.0.get() {
        let remaining = maximum.0.get().saturating_sub(output.len());
        let read_length = remaining.min(buffer.len());
        let target = buffer
            .get_mut(..read_length)
            .ok_or(super::ChildProcessError::DiagnosticRange)?;
        let read = tokio::io::AsyncReadExt::read(&mut reader, target)
            .await
            .map_err(super::ChildProcessIoError::from)
            .map_err(super::ChildProcessError::DiagnosticIo)?;
        if read == constants_usize::ZERO {
            break;
        }
        let read_bytes = buffer
            .get(..read)
            .ok_or(super::ChildProcessError::DiagnosticRange)?;
        output.extend_from_slice(read_bytes);
    }
    Ok(super::ChildDiagnostic::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(output),
    ))
}
