use super::{ListOffset, ListRowsPresence, ListTotalSource, WindowTotalPresence};

#[must_use]
pub const fn resolve_list_total_source(
    offset: ListOffset,
    rows: ListRowsPresence,
    window_total: WindowTotalPresence,
) -> ListTotalSource {
    match (rows, window_total, offset.0) {
        (ListRowsPresence::Present, WindowTotalPresence::Present, _) => ListTotalSource::Window,
        (ListRowsPresence::Empty, _, constants_i64::ZERO) => ListTotalSource::Zero,
        (ListRowsPresence::Empty | ListRowsPresence::Present, _, _) => ListTotalSource::CountQuery,
    }
}
