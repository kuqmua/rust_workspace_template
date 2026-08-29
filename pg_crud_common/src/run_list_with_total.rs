pub async fn run_list_with_total<
    Item,
    Error,
    FetchList,
    FetchListFuture,
    FetchCount,
    FetchCountFuture,
>(
    offset: crate::list_offset::ListOffset,
    fetch_list: FetchList,
    fetch_count: FetchCount,
) -> Result<crate::list_page::ListPage<Item>, Error>
where
    FetchList: FnOnce() -> FetchListFuture,
    FetchListFuture: Future<Output = Result<crate::list_rows::ListRows<Item>, Error>>,
    FetchCount: FnOnce() -> FetchCountFuture,
    FetchCountFuture: Future<Output = Result<crate::list_total::ListTotal, Error>>,
{
    let rows = fetch_list().await?;
    let rows_presence = if rows.items.0.is_empty() {
        crate::list_rows_presence::ListRowsPresence::Empty
    } else {
        crate::list_rows_presence::ListRowsPresence::Present
    };
    let window_presence = if rows.window_total.is_some() {
        crate::window_total_presence::WindowTotalPresence::Present
    } else {
        crate::window_total_presence::WindowTotalPresence::Absent
    };
    let total = match crate::resolve_list_total_source::resolve_list_total_source(
        offset,
        rows_presence,
        window_presence,
    ) {
        crate::list_total_source::ListTotalSource::CountQuery => fetch_count().await?,
        crate::list_total_source::ListTotalSource::Window => rows
            .window_total
            .unwrap_or_else(|| crate::list_total::ListTotal::from(constants_u32::ZERO)),
        crate::list_total_source::ListTotalSource::Zero => {
            crate::list_total::ListTotal::from(constants_u32::ZERO)
        }
    };
    Ok(crate::list_page::ListPage {
        items: rows.items,
        total,
    })
}
