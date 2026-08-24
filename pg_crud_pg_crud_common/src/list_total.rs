#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListOffset(i64);
impl From<crate::PaginationOffset> for ListOffset {
    fn from(value: crate::PaginationOffset) -> Self {
        Self(value.get())
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListRowsPresence {
    Empty,
    Present,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowTotalPresence {
    Absent,
    Present,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListTotalSource {
    CountQuery,
    Window,
    Zero,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::IntoInnerFrom)]
pub struct ListTotal(i64);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("list total must not be negative")]
pub struct ListTotalError;
impl TryFrom<i64> for ListTotal {
    type Error = ListTotalError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0i64 {
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

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct ListItems<Item>(Vec<Item>);

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct ListPage<Item> {
    items: ListItems<Item>,
    total: ListTotal,
}
impl<Item> ListPage<Item> {
    #[must_use]
    pub const fn items(&self) -> &[Item] {
        self.items.0.as_slice()
    }

    #[must_use]
    pub const fn total(&self) -> ListTotal {
        self.total
    }
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct ListRows<Item> {
    items: ListItems<Item>,
    window_total: Option<ListTotal>,
}
impl<Item> ListRows<Item> {
    #[must_use]
    pub const fn new(items: ListItems<Item>, window_total: Option<ListTotal>) -> Self {
        Self {
            items,
            window_total,
        }
    }
}

pub async fn run_list_with_total<
    Item,
    Error,
    FetchList,
    FetchListFuture,
    FetchCount,
    FetchCountFuture,
>(
    offset: ListOffset,
    fetch_list: FetchList,
    fetch_count: FetchCount,
) -> Result<ListPage<Item>, Error>
where
    FetchList: FnOnce() -> FetchListFuture,
    FetchListFuture: Future<Output = Result<ListRows<Item>, Error>>,
    FetchCount: FnOnce() -> FetchCountFuture,
    FetchCountFuture: Future<Output = Result<ListTotal, Error>>,
{
    let rows = fetch_list().await?;
    let rows_presence = if rows.items.0.is_empty() {
        ListRowsPresence::Empty
    } else {
        ListRowsPresence::Present
    };
    let window_presence = if rows.window_total.is_some() {
        WindowTotalPresence::Present
    } else {
        WindowTotalPresence::Absent
    };
    let total = match list_total_source(offset, rows_presence, window_presence) {
        ListTotalSource::CountQuery => fetch_count().await?,
        ListTotalSource::Window => rows.window_total.unwrap_or_else(|| ListTotal::from(0u32)),
        ListTotalSource::Zero => ListTotal::from(0u32),
    };
    Ok(ListPage {
        items: rows.items,
        total,
    })
}

#[must_use]
pub const fn list_total_source(
    offset: ListOffset,
    rows: ListRowsPresence,
    window_total: WindowTotalPresence,
) -> ListTotalSource {
    match (rows, window_total, offset.0) {
        (ListRowsPresence::Present, WindowTotalPresence::Present, _) => ListTotalSource::Window,
        (ListRowsPresence::Empty, _, 0i64) => ListTotalSource::Zero,
        (ListRowsPresence::Empty | ListRowsPresence::Present, _, _) => ListTotalSource::CountQuery,
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
    fn list_total_rejects_negative_and_accepts_zero() {
        assert_eq!(
            super::ListTotal::try_from(-1i64),
            Err(super::ListTotalError)
        );
        assert_eq!(
            i64::from(super::ListTotal::try_from(0i64).expect(
                "93f8d6c5 list_total_rejects_negative_and_accepts_zero invariant must hold"
            )),
            0i64
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
            crate::PaginationOffset::from(0i32).into(),
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
            crate::PaginationOffset::from(0i32).into(),
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
        assert_eq!(i64::from(page.total()), 0i64);
        assert!(!count_called.get());
    }

    #[test]
    fn run_list_uses_count_for_later_or_windowless_pages() {
        let count_calls = std::cell::Cell::new(0usize);
        let later_page = block_on(super::run_list_with_total(
            crate::PaginationOffset::from(1i32).into(),
            async || {
                Ok::<_, ()>(super::ListRows::new(
                    super::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            || {
                count_calls.set(count_calls.get() + 1usize);
                async { Ok::<_, ()>(super::ListTotal::from(17u32)) }
            },
        ))
        .expect("27f9f3eb run_list_uses_count_for_later_or_windowless_pages invariant must hold");
        assert_eq!(i64::from(later_page.total()), 17i64);
        assert_eq!(count_calls.get(), 1usize);

        let windowless_page = block_on(super::run_list_with_total(
            crate::PaginationOffset::from(0i32).into(),
            async || {
                Ok::<_, ()>(super::ListRows::new(
                    super::ListItems::from(vec![1u8]),
                    None,
                ))
            },
            || {
                count_calls.set(count_calls.get() + 1usize);
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
            crate::PaginationOffset::from(0i32).into(),
            async || Err::<super::ListRows<u8>, _>("list"),
            async || Ok::<_, &str>(super::ListTotal::from(0u32)),
        ))
        .expect_err("729e9c33");
        assert_eq!(list_error, "list");
        let count_error = block_on(super::run_list_with_total(
            crate::PaginationOffset::from(1i32).into(),
            async || {
                Ok::<_, &str>(super::ListRows::new(
                    super::ListItems::from(Vec::<u8>::new()),
                    None,
                ))
            },
            async || Err::<super::ListTotal, _>("count"),
        ))
        .expect_err("50820e0d");
        assert_eq!(count_error, "count");
    }

    #[test]
    fn window_total_avoids_separate_count_query() {
        assert_eq!(
            super::list_total_source(
                crate::PaginationOffset::from(0i32).into(),
                super::ListRowsPresence::Present,
                super::WindowTotalPresence::Present,
            ),
            super::ListTotalSource::Window
        );
    }

    #[test]
    fn empty_first_page_has_zero_total_without_count_query() {
        assert_eq!(
            super::list_total_source(
                crate::PaginationOffset::from(0i32).into(),
                super::ListRowsPresence::Empty,
                super::WindowTotalPresence::Absent,
            ),
            super::ListTotalSource::Zero
        );
    }

    #[test]
    fn empty_later_page_requires_count_query() {
        assert_eq!(
            super::list_total_source(
                crate::PaginationOffset::from(1i32).into(),
                super::ListRowsPresence::Empty,
                super::WindowTotalPresence::Absent,
            ),
            super::ListTotalSource::CountQuery
        );
    }
}
