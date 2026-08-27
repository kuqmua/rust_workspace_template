#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "list_total_source/list_total_source.rs"]
mod list_total_source;

pub use list_total_source::ListTotalSource;

#[must_use]
pub const fn list_total_source(
    offset: super::ListOffset,
    rows: super::ListRowsPresence,
    window_total: super::WindowTotalPresence,
) -> ListTotalSource {
    match (rows, window_total, offset.0) {
        (super::ListRowsPresence::Present, super::WindowTotalPresence::Present, _) => {
            ListTotalSource::Window
        }
        (super::ListRowsPresence::Empty, _, constants_i64::ZERO) => ListTotalSource::Zero,
        (super::ListRowsPresence::Empty | super::ListRowsPresence::Present, _, _) => {
            ListTotalSource::CountQuery
        }
    }
}
