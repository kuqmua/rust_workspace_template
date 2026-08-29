#[must_use]
pub const fn resolve_list_total_source(
    offset: crate::list_offset::ListOffset,
    rows: crate::list_rows_presence::ListRowsPresence,
    window_total: crate::window_total_presence::WindowTotalPresence,
) -> crate::list_total_source::ListTotalSource {
    match (rows, window_total, offset.0) {
        (
            crate::list_rows_presence::ListRowsPresence::Present,
            crate::window_total_presence::WindowTotalPresence::Present,
            _,
        ) => crate::list_total_source::ListTotalSource::Window,
        (crate::list_rows_presence::ListRowsPresence::Empty, _, constants_i64::ZERO) => {
            crate::list_total_source::ListTotalSource::Zero
        }
        (
            crate::list_rows_presence::ListRowsPresence::Empty
            | crate::list_rows_presence::ListRowsPresence::Present,
            _,
            _,
        ) => crate::list_total_source::ListTotalSource::CountQuery,
    }
}
