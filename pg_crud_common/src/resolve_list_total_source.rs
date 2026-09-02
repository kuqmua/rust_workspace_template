#[must_use]
pub const fn resolve_list_total_source(
    list_offset: crate::list_offset::ListOffset,
    list_rows_presence: crate::list_rows_presence::ListRowsPresence,
    window_total_presence: crate::window_total_presence::WindowTotalPresence,
) -> crate::list_total_source::ListTotalSource {
    match (
        list_rows_presence,
        window_total_presence,
        *list_offset.get_inner(),
    ) {
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
