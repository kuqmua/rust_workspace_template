#[cfg(test)]
mod tests {
    #[test]
    fn test_list_total_source_matrix_preserves_items_and_count_call_policy() {
        assert!(
            [
                (0i64, false, false, 0i64, 0usize),
                (0i64, false, true, 0i64, 0usize),
                (0i64, true, false, 99i64, 1usize),
                (0i64, true, true, 11i64, 0usize),
                (1i64, false, false, 99i64, 1usize),
                (1i64, false, true, 99i64, 1usize),
                (1i64, true, false, 99i64, 1usize),
                (1i64, true, true, 11i64, 0usize),
            ]
            .into_iter()
            .all(
                |(offset, has_items, has_window_total, expected_total, expected_calls)| {
                    let count_calls = std::cell::Cell::new(0usize);
                    let items = if has_items {
                        vec![crate::query_part_increment::QueryPartIncrement::from(7u64)]
                    } else {
                        Vec::new()
                    };
                    let rows = crate::list_rows::ListRows::new(
                        crate::list_items::ListItems::from(items),
                        has_window_total.then(|| crate::list_total::ListTotal::from(11u32)),
                    );
                    let mut future =
                        std::pin::pin!(crate::run_list_with_total::run_list_with_total(
                            crate::list_offset::ListOffset::from(
                                crate::pagination_offset::PaginationOffset::from(offset),
                            ),
                            || std::future::ready(
                                Ok::<_, crate::list_total_error::ListTotalError>(rows)
                            ),
                            || {
                                count_calls.set(count_calls.get() + 1usize);
                                std::future::ready(
                                    Ok::<_, crate::list_total_error::ListTotalError>(
                                        crate::list_total::ListTotal::from(99u32),
                                    ),
                                )
                            },
                        ));
                    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
                    matches!(Future::poll(future.as_mut(), &mut context),
                        std::task::Poll::Ready(Ok(page))
                            if i64::from(page.total()) == expected_total
                            && page.items().len() == usize::from(has_items)
                                && page.items().iter().all(|item| item.get() == 7u64)
                                && count_calls.get() == expected_calls
                    )
                }
            )
        );
    }
}
