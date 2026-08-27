pub async fn run_list_with_total<
    Item,
    Error,
    FetchList,
    FetchListFuture,
    FetchCount,
    FetchCountFuture,
>(
    offset: super::ListOffset,
    fetch_list: FetchList,
    fetch_count: FetchCount,
) -> Result<super::ListPage<Item>, Error>
where
    FetchList: FnOnce() -> FetchListFuture,
    FetchListFuture: Future<Output = Result<super::ListRows<Item>, Error>>,
    FetchCount: FnOnce() -> FetchCountFuture,
    FetchCountFuture: Future<Output = Result<super::ListTotal, Error>>,
{
    let rows = fetch_list().await?;
    let rows_presence = if rows.items.0.is_empty() {
        super::ListRowsPresence::Empty
    } else {
        super::ListRowsPresence::Present
    };
    let window_presence = if rows.window_total.is_some() {
        super::WindowTotalPresence::Present
    } else {
        super::WindowTotalPresence::Absent
    };
    let total = match super::list_total_source(offset, rows_presence, window_presence) {
        super::ListTotalSource::CountQuery => fetch_count().await?,
        super::ListTotalSource::Window => rows
            .window_total
            .unwrap_or_else(|| super::ListTotal::from(constants_u32::ZERO)),
        super::ListTotalSource::Zero => super::ListTotal::from(constants_u32::ZERO),
    };
    Ok(super::ListPage {
        items: rows.items,
        total,
    })
}
