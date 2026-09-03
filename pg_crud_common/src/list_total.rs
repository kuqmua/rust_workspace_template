#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct ListTotal(i64);

impl TryFrom<i64> for ListTotal {
    type Error = crate::list_total_error::ListTotalError;

    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        if i64 < constants_i64::ZERO {
            Err(crate::list_total_error::ListTotalError::Negative)
        } else {
            Ok(Self(i64))
        }
    }
}

impl From<u32> for ListTotal {
    fn from(u32: u32) -> Self {
        Self(i64::from(u32))
    }
}
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
    fn test_list_total_rejects_negative_and_accepts_zero() {
        assert_eq!(
            crate::list_total::ListTotal::try_from(-constants_i64::ONE),
            Err(crate::list_total_error::ListTotalError::Negative)
        );
        assert_eq!(
            i64::from(
                crate::list_total::ListTotal::try_from(constants_i64::ZERO)
                    .expect(constants_str::DIAGNOSTIC_93F8D6C5)
            ),
            constants_i64::ZERO
        );
        assert_eq!(
            i64::from(crate::list_total::ListTotal::from(u32::MAX)),
            i64::from(u32::MAX)
        );
    }

    #[test]
    fn test_run_list_uses_window_total_without_calling_count() {
        let count_called = std::cell::Cell::new(false);
        let window_page = block_on(crate::run_list_with_total::run_list_with_total(
            crate::pagination_offset::PaginationOffset::from(constants_i32::ZERO).into(),
            async || {
                Ok::<_, ()>(crate::list_rows::ListRows::new(
                    crate::list_items::ListItems::from(vec![7u8]),
                    Some(
                        crate::list_total::ListTotal::try_from(11i64)
                            .expect(constants_str::DIAGNOSTIC_8D096C08),
                    ),
                ))
            },
            || {
                count_called.set(true);
                async { Ok::<_, ()>(crate::list_total::ListTotal::from(99u32)) }
            },
        ))
        .expect(constants_str::DIAGNOSTIC_CBA64F03);
        assert_eq!(window_page.items(), &[7u8]);
        assert_eq!(i64::from(window_page.total()), 11i64);
        assert!(!count_called.get());
    }

    #[test]
    fn test_run_list_uses_zero_for_empty_first_page_without_calling_count() {
        let count_called = std::cell::Cell::new(false);
        let page = block_on(crate::run_list_with_total::run_list_with_total(
            crate::pagination_offset::PaginationOffset::from(constants_i32::ZERO).into(),
            async || {
                Ok::<_, ()>(crate::list_rows::ListRows::new(
                    crate::list_items::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            || {
                count_called.set(true);
                async { Ok::<_, ()>(crate::list_total::ListTotal::from(99u32)) }
            },
        ))
        .expect(constants_str::DIAGNOSTIC_704C4827);
        assert!(page.items().is_empty());
        assert_eq!(i64::from(page.total()), constants_i64::ZERO);
        assert!(!count_called.get());
    }

    #[test]
    fn test_run_list_uses_count_for_later_or_windowless_pages() {
        let count_calls = std::cell::Cell::new(constants_usize::ZERO);
        let later_page = block_on(crate::run_list_with_total::run_list_with_total(
            crate::pagination_offset::PaginationOffset::from(1i32).into(),
            async || {
                Ok::<_, ()>(crate::list_rows::ListRows::new(
                    crate::list_items::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            || {
                count_calls.set(count_calls.get() + constants_usize::ONE);
                async { Ok::<_, ()>(crate::list_total::ListTotal::from(17u32)) }
            },
        ))
        .expect(constants_str::DIAGNOSTIC_27F9F3EB);
        assert_eq!(i64::from(later_page.total()), 17i64);
        assert_eq!(count_calls.get(), constants_usize::ONE);

        let windowless_page = block_on(crate::run_list_with_total::run_list_with_total(
            crate::pagination_offset::PaginationOffset::from(constants_i32::ZERO).into(),
            async || {
                Ok::<_, ()>(crate::list_rows::ListRows::new(
                    crate::list_items::ListItems::from(vec![1u8]),
                    None,
                ))
            },
            || {
                count_calls.set(count_calls.get() + constants_usize::ONE);
                async { Ok::<_, ()>(crate::list_total::ListTotal::from(23u32)) }
            },
        ))
        .expect(constants_str::DIAGNOSTIC_0FF9C45E);
        assert_eq!(i64::from(windowless_page.total()), 23i64);
        assert_eq!(count_calls.get(), 2usize);
    }

    #[test]
    fn test_run_list_propagates_list_and_count_errors() {
        let list_error = block_on(crate::run_list_with_total::run_list_with_total(
            crate::pagination_offset::PaginationOffset::from(constants_i32::ZERO).into(),
            async || Err::<crate::list_rows::ListRows<u8>, _>(constants_str::VALUE_A330395C),
            async || Ok::<_, &str>(crate::list_total::ListTotal::from(constants_u32::ZERO)),
        ))
        .expect_err(constants_str::VALUE_09221460);
        assert_eq!(list_error, constants_str::VALUE_A330395C);
        let count_error = block_on(crate::run_list_with_total::run_list_with_total(
            crate::pagination_offset::PaginationOffset::from(1i32).into(),
            async || {
                Ok::<_, &str>(crate::list_rows::ListRows::new(
                    crate::list_items::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            async || Err::<crate::list_total::ListTotal, _>(constants_str::VALUE_6C35493A),
        ))
        .expect_err(constants_str::VALUE_20016253);
        assert_eq!(count_error, constants_str::VALUE_6C35493A);
    }

    #[test]
    fn test_window_total_avoids_separate_count_query() {
        assert_eq!(
            crate::resolve_list_total_source::resolve_list_total_source(
                crate::pagination_offset::PaginationOffset::from(constants_i32::ZERO).into(),
                crate::list_rows_presence::ListRowsPresence::Present,
                crate::window_total_presence::WindowTotalPresence::Present,
            ),
            crate::list_total_source::ListTotalSource::Window
        );
    }

    #[test]
    fn test_empty_first_page_has_zero_total_without_count_query() {
        assert_eq!(
            crate::resolve_list_total_source::resolve_list_total_source(
                crate::pagination_offset::PaginationOffset::from(constants_i32::ZERO).into(),
                crate::list_rows_presence::ListRowsPresence::Empty,
                crate::window_total_presence::WindowTotalPresence::Absent,
            ),
            crate::list_total_source::ListTotalSource::Zero
        );
    }

    #[test]
    fn test_empty_later_page_requires_count_query() {
        assert_eq!(
            crate::resolve_list_total_source::resolve_list_total_source(
                crate::pagination_offset::PaginationOffset::from(1i32).into(),
                crate::list_rows_presence::ListRowsPresence::Empty,
                crate::window_total_presence::WindowTotalPresence::Absent,
            ),
            crate::list_total_source::ListTotalSource::CountQuery
        );
    }
}
