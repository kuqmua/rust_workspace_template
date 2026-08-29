#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
)]
pub struct ListTotal(i64);

impl TryFrom<i64> for ListTotal {
    type Error = ListTotalError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < constants_i64::ZERO {
            Err(ListTotalError)
        } else {
            Ok(Self(value))
        }
    }
}

impl From<u32> for ListTotal {
    fn from(value: u32) -> Self {
        Self(i64::from(value))
    }
}

pub use super::list_items::ListItems;
pub use super::list_offset::ListOffset;
pub use super::list_page::ListPage;
pub use super::list_rows::ListRows;
pub use super::list_rows_presence::ListRowsPresence;
pub use super::list_total_error::ListTotalError;
pub use super::list_total_source::ListTotalSource;
pub use super::resolve_list_total_source::resolve_list_total_source;
pub use super::run_list_with_total::run_list_with_total;
pub use super::window_total_presence::WindowTotalPresence;
#[cfg(test)]
mod tests {
    fn block_on<Output>(input_future: impl Future<Output = Output>) -> Output {
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        let mut pinned_future = std::pin::pin!(input_future);
        loop {
            match Future::poll(pinned_future.as_mut(), &mut context) {
                std::task::Poll::Ready(output) => return output,
                std::task::Poll::Pending => std::thread::yield_now(),
            }
        }
    }
    #[test]
    fn list_total_rejects_negative_and_accepts_zero() {
        assert_eq!(
            super::ListTotal::try_from(-constants_i64::ONE),
            Err(super::ListTotalError)
        );
        assert_eq!(
            i64::from(super::ListTotal::try_from(constants_i64::ZERO).expect(
                "93f8d6c5 list_total_rejects_negative_and_accepts_zero invariant must hold"
            )),
            constants_i64::ZERO
        );
        assert_eq!(
            i64::from(super::ListTotal::from(u32::MAX)),
            i64::from(u32::MAX)
        );
    }

    #[test]
    fn run_list_uses_window_total_without_calling_count() {
        let count_called = std::cell::Cell::new(false);
        let window_page = block_on(super::run_list_with_total(
            crate::domain_types::PaginationOffset::from(constants_i32::ZERO).into(),
            async || {
                Ok::<_, ()>(super::ListRows::new(
                    super::ListItems::from(vec![7u8]),
                    Some(super::ListTotal::try_from(11i64).expect("8d096c08 run_list_uses_window_total_without_calling_count invariant must hold")),
                ))
            },
            || {
                count_called.set(true);
                async { Ok::<_, ()>(super::ListTotal::from(99u32)) }
            },
        ))
        .expect("cba64f03 run_list_uses_window_total_without_calling_count invariant must hold");
        assert_eq!(window_page.items(), &[7u8]);
        assert_eq!(i64::from(window_page.total()), 11i64);
        assert!(!count_called.get());
    }

    #[test]
    fn run_list_uses_zero_for_empty_first_page_without_calling_count() {
        let count_called = std::cell::Cell::new(false);
        let page = block_on(super::run_list_with_total(
            crate::domain_types::PaginationOffset::from(constants_i32::ZERO).into(),
            async || {
                Ok::<_, ()>(super::ListRows::new(
                    super::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            || {
                count_called.set(true);
                async { Ok::<_, ()>(super::ListTotal::from(99u32)) }
            },
        ))
        .expect("704c4827 run_list_uses_zero_for_empty_first_page_without_calling_count invariant must hold");
        assert!(page.items().is_empty());
        assert_eq!(i64::from(page.total()), constants_i64::ZERO);
        assert!(!count_called.get());
    }

    #[test]
    fn run_list_uses_count_for_later_or_windowless_pages() {
        let count_calls = std::cell::Cell::new(constants_usize::ZERO);
        let later_page = block_on(super::run_list_with_total(
            crate::domain_types::PaginationOffset::from(1i32).into(),
            async || {
                Ok::<_, ()>(super::ListRows::new(
                    super::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            || {
                count_calls.set(count_calls.get() + constants_usize::ONE);
                async { Ok::<_, ()>(super::ListTotal::from(17u32)) }
            },
        ))
        .expect("27f9f3eb run_list_uses_count_for_later_or_windowless_pages invariant must hold");
        assert_eq!(i64::from(later_page.total()), 17i64);
        assert_eq!(count_calls.get(), constants_usize::ONE);

        let windowless_page = block_on(super::run_list_with_total(
            crate::domain_types::PaginationOffset::from(constants_i32::ZERO).into(),
            async || {
                Ok::<_, ()>(super::ListRows::new(
                    super::ListItems::from(vec![1u8]),
                    None,
                ))
            },
            || {
                count_calls.set(count_calls.get() + constants_usize::ONE);
                async { Ok::<_, ()>(super::ListTotal::from(23u32)) }
            },
        ))
        .expect("0ff9c45e run_list_uses_count_for_later_or_windowless_pages invariant must hold");
        assert_eq!(i64::from(windowless_page.total()), 23i64);
        assert_eq!(count_calls.get(), 2usize);
    }

    #[test]
    fn run_list_propagates_list_and_count_errors() {
        let list_error = block_on(super::run_list_with_total(
            crate::domain_types::PaginationOffset::from(constants_i32::ZERO).into(),
            async || Err::<super::ListRows<u8>, _>(constants_str::VALUE_A330395C),
            async || Ok::<_, &str>(super::ListTotal::from(constants_u32::ZERO)),
        ))
        .expect_err(constants_str::VALUE_09221460);
        assert_eq!(list_error, "list");
        let count_error = block_on(super::run_list_with_total(
            crate::domain_types::PaginationOffset::from(1i32).into(),
            async || {
                Ok::<_, &str>(super::ListRows::new(
                    super::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            async || Err::<super::ListTotal, _>(constants_str::VALUE_6C35493A),
        ))
        .expect_err(constants_str::VALUE_20016253);
        assert_eq!(count_error, "count");
    }

    #[test]
    fn window_total_avoids_separate_count_query() {
        assert_eq!(
            super::resolve_list_total_source(
                crate::domain_types::PaginationOffset::from(constants_i32::ZERO).into(),
                super::ListRowsPresence::Present,
                super::WindowTotalPresence::Present,
            ),
            super::ListTotalSource::Window
        );
    }

    #[test]
    fn empty_first_page_has_zero_total_without_count_query() {
        assert_eq!(
            super::resolve_list_total_source(
                crate::domain_types::PaginationOffset::from(constants_i32::ZERO).into(),
                super::ListRowsPresence::Empty,
                super::WindowTotalPresence::Absent,
            ),
            super::ListTotalSource::Zero
        );
    }

    #[test]
    fn empty_later_page_requires_count_query() {
        assert_eq!(
            super::resolve_list_total_source(
                crate::domain_types::PaginationOffset::from(1i32).into(),
                super::ListRowsPresence::Empty,
                super::WindowTotalPresence::Absent,
            ),
            super::ListTotalSource::CountQuery
        );
    }
}
