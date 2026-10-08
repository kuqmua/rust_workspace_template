#[test]
fn test_between_json_shapes_normalize_to_exact_inclusive_bound_fields() {
    assert!(
        [(i32::MIN, i32::MAX), (0i32, 0i32), (1i32, 2i32)]
            .into_iter()
            .all(|(start, end)| {
                let expected = serde_json::json!({
                    (stringify!(start)): start,
                    (stringify!(end)): end,
                });
                let with_unknown = serde_json::json!({
                    (stringify!(start)): start,
                    (stringify!(end)): end,
                    (stringify!(unknown)): {(stringify!(unknown)): [null, false, 99i32]},
                });
                [
                    expected.clone(),
                    serde_json::json!([start, end]),
                    with_unknown,
                ]
                .into_iter()
                .all(|input| {
                    serde_json::from_value::<where_filters::between::Between<i32>>(input).is_ok_and(
                        |between| {
                            serde_json::to_value(between).is_ok_and(|actual| actual == expected)
                        },
                    )
                })
            })
    );
    let default = <where_filters::between::Between<
        pg_crud_common::unsigned_part_of_i32::UnsignedPartOfI32,
    > as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element(
    );
    assert!(serde_json::to_value(default).is_ok_and(|actual| actual
        == serde_json::json!({
            (stringify!(start)): 0i32,
            (stringify!(end)): 0i32,
        })));
}

#[test]
fn test_between_json_rejects_reversed_bounds_extra_sequence_items_and_wrong_shapes() {
    assert!(
        [
            serde_json::json!({(stringify!(start)): 2i32, (stringify!(end)): 1i32}),
            serde_json::json!([2i32, 1i32]),
            serde_json::json!([1i32, 2i32, 3i32]),
            serde_json::json!({(stringify!(start)): null, (stringify!(end)): 2i32}),
            serde_json::json!({(stringify!(start)): 1i32, (stringify!(end)): false}),
            serde_json::json!(null),
            serde_json::json!(false),
            serde_json::json!(1i32),
        ]
        .into_iter()
        .all(|input| {
            serde_json::from_value::<where_filters::between::Between<i32>>(input)
                .is_err_and(|error| error.is_data())
        })
    );
}
