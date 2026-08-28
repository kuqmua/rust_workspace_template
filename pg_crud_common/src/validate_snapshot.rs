pub(crate) fn validate_snapshot<Snapshot, Error, Mismatch>(
    expected: Snapshot,
    observed: Snapshot,
    mismatch: Mismatch,
) -> Result<(), Error>
where
    Snapshot: PartialEq,
    Mismatch: FnOnce(Snapshot, Snapshot) -> Error,
{
    if expected == observed {
        Ok(())
    } else {
        Err(mismatch(expected, observed))
    }
}
