#[test]
fn test_maximum_rejects_zero_and_accepts_positive_values() {
    assert_eq!(
        server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(
            constants_usize::ZERO
        ),
        Err(server_runtime_core::resource_budget_config_error::ResourceBudgetConfigError::Zero)
    );
    let maximum = std::num::NonZeroUsize::new(constants_usize::ONE)
        .expect(constants_str::DIAGNOSTIC_9E83081E);
    assert_eq!(
        server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(
            constants_usize::ONE
        )
        .expect(constants_str::DIAGNOSTIC_19C82820),
        server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::from(maximum)
    );
}

#[test]
fn test_reservations_are_bounded_and_released() {
    let budget = server_runtime_core::resource_budget::ResourceBudget::new(
        server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(5usize)
            .expect(constants_str::DIAGNOSTIC_0C6362A4),
    );
    let first = budget
        .reserve(server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize))
        .expect(constants_str::DIAGNOSTIC_3BFEB37C);
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize)
    );
    assert_eq!(
        budget
            .reserve(
                server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize)
            )
            .expect_err(constants_str::VALUE_3C31187B),
        server_runtime_core::resource_budget_reserve_error::ResourceBudgetReserveError::Exhausted
    );
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize)
    );
    let second = budget
        .reserve(server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(2usize))
        .expect(constants_str::DIAGNOSTIC_D86085DB);
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(5usize)
    );
    drop(first);
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(2usize)
    );
    drop(second);
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::ZERO
        )
    );
}

#[test]
fn test_overflow_does_not_change_reserved_count() {
    let budget = server_runtime_core::resource_budget::ResourceBudget::new(
        server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(usize::MAX)
            .expect(constants_str::DIAGNOSTIC_65F2F229),
    );
    let reservation = budget
        .reserve(
            server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(
                constants_usize::ONE,
            ),
        )
        .expect(constants_str::DIAGNOSTIC_1A2BB321);
    assert_eq!(
        budget
            .reserve(
                server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(usize::MAX)
            )
            .expect_err(constants_str::VALUE_E317C775),
        server_runtime_core::resource_budget_reserve_error::ResourceBudgetReserveError::Overflow
    );
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::ONE
        )
    );
    drop(reservation);
}

#[test]
fn test_concurrent_reservations_never_exceed_maximum() {
    let budget = server_runtime_core::resource_budget::ResourceBudget::new(
        server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(5usize)
            .expect(constants_str::DIAGNOSTIC_57A61CA4),
    );
    let start = std::sync::Arc::new(std::sync::Barrier::new(3usize));
    let finish = std::sync::Arc::new(std::sync::Barrier::new(3usize));
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::scope(|scope| {
        let left_budget = budget.clone();
        let left_start = std::sync::Arc::clone(&start);
        let left_finish = std::sync::Arc::clone(&finish);
        let left_tx = tx.clone();
        let _left_task = scope.spawn(move || {
            let _start_result = left_start.wait();
            let reservation = left_budget.reserve(
                server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize),
            );
            left_tx
                .send(reservation.is_ok())
                .expect(constants_str::DIAGNOSTIC_B048535E);
            let _finish_result = left_finish.wait();
            drop(reservation);
        });
        let right_budget = budget.clone();
        let right_start = std::sync::Arc::clone(&start);
        let right_finish = std::sync::Arc::clone(&finish);
        let right_tx = tx.clone();
        let _right_task = scope.spawn(move || {
            let _start_result = right_start.wait();
            let reservation = right_budget.reserve(
                server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize),
            );
            right_tx
                .send(reservation.is_ok())
                .expect(constants_str::DIAGNOSTIC_CD734995);
            let _finish_result = right_finish.wait();
            drop(reservation);
        });
        let _start_result = start.wait();
        let outcomes = [
            rx.recv().expect(constants_str::DIAGNOSTIC_7393AFCA),
            rx.recv().expect(constants_str::DIAGNOSTIC_67824B65),
        ];
        assert_eq!(
            outcomes.into_iter().filter(|value| *value).count(),
            constants_usize::ONE
        );
        assert_eq!(
            budget.reserved(),
            server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(3usize)
        );
        let _finish_result = finish.wait();
    });
    assert_eq!(
        budget.reserved(),
        server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::ZERO
        )
    );
}
