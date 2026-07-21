#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListOffset(i64);
impl From<crate::PaginationOffset> for ListOffset {
    fn from(value: crate::PaginationOffset) -> Self {
        Self(value.get())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListRowsPresence {
    Empty,
    Present,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowTotalPresence {
    Absent,
    Present,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListTotalSource {
    CountQuery,
    Window,
    Zero,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListTotal(i64);
impl From<ListTotal> for i64 {
    fn from(value: ListTotal) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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

#[derive(Clone, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct ListItems<Item>(Vec<Item>);

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
        ListTotalSource::Window => rows.window_total.unwrap_or(ListTotal::from(0i64)),
        ListTotalSource::Zero => ListTotal::from(0i64),
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
