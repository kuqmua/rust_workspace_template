#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn run_where_filters() -> Result<(), ()> {
    (|| {
        let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
        let where_filters_bounded_vec =
            match where_filters::domain_types::BoundedVec::<i32, 64>::try_from(where_filters_values)
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "allocation_workload=where_filters_query_part status=setup_failed error={error:?}"
                    );
                    return Err(());
                }
            };
        let output_bytes = (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT)
                    .try_fold(constants_usize::ZERO, |series_accumulator, _| {
                        (0..crate::domain_types::MEASURE_REPEAT_COUNT).try_fold(
                            series_accumulator,
                            |accumulator, _| {
                                let mut increment = constants_u64::ZERO;
                                match where_filters_bounded_vec.pg_type_query_part(
                                    &mut increment,
                                    pg_crud_common::domain_types::SqlColumnRef::from(
                                        &constants_str::COLUMN,
                                    ),
                                    pg_crud_common::domain_types::AddOperator::from(false),
                                ) {
                                    Ok(fragment) => {
                                        Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                    }
                                    Err(error) => {
                                        eprintln!(
                                            "allocation_workload=where_filters_query_part status=failed error={error:?}"
                                        );
                                        Err(())
                                    }
                                }
                            },
                        )
                    })?;
        println!(
            "allocation_workload=where_filters_query_part series_count={series_count} repeat_count={repeat_count} output_bytes={output_bytes}",
            series_count = crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
            repeat_count = crate::domain_types::MEASURE_REPEAT_COUNT,
        );
        Ok(())
    })()
}
