pub(crate) fn validate_snapshot<Snapshot>(
    expected: Snapshot,
    observed: Snapshot,
) -> Result<(), crate::db_schema_conformance_error::DbSchemaConformanceError>
where
    Snapshot: crate::snapshot_mismatch::SnapshotMismatch,
{
    if expected == observed {
        Ok(())
    } else {
        Err(Snapshot::mismatch(expected, observed))
    }
}
