#[test]
fn maximum_rejects_zero_and_accepts_positive_values() {
    assert_eq!(
        super::super::ResourceBudgetMaximum::try_from(constants_usize::ZERO),
        Err(super::super::ResourceBudgetConfigError)
    );
    let maximum = std::num::NonZeroUsize::new(constants_usize::ONE)
        .expect("9e83081e maximum_rejects_zero_and_accepts_positive_values invariant must hold");
    assert_eq!(
        super::super::ResourceBudgetMaximum::try_from(constants_usize::ONE).expect(
            "19c82820 maximum_rejects_zero_and_accepts_positive_values invariant must hold"
        ),
        super::super::ResourceBudgetMaximum::from(maximum)
    );
}

#[test]
fn reservations_are_bounded_and_released() {
    let budget = super::super::ResourceBudget::new(
        super::super::ResourceBudgetMaximum::try_from(5usize)
            .expect("0c6362a4 reservations_are_bounded_and_released invariant must hold"),
    );
    let first = budget
        .reserve(super::super::ResourceBudgetAmount::from(3usize))
        .expect("3bfeb37c reservations_are_bounded_and_released invariant must hold");
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(3usize)
    );
    assert_eq!(
        budget
            .reserve(super::super::ResourceBudgetAmount::from(3usize))
            .expect_err("3c31187b"),
        super::super::ResourceBudgetReserveError::Exhausted
    );
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(3usize)
    );
    let second = budget
        .reserve(super::super::ResourceBudgetAmount::from(2usize))
        .expect("d86085db reservations_are_bounded_and_released invariant must hold");
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(5usize)
    );
    drop(first);
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(2usize)
    );
    drop(second);
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(constants_usize::ZERO)
    );
}

#[test]
fn overflow_does_not_change_reserved_count() {
    let budget = super::super::ResourceBudget::new(
        super::super::ResourceBudgetMaximum::try_from(usize::MAX)
            .expect("65f2f229 overflow_does_not_change_reserved_count invariant must hold"),
    );
    let reservation = budget
        .reserve(super::super::ResourceBudgetAmount::from(
            constants_usize::ONE,
        ))
        .expect("1a2bb321 overflow_does_not_change_reserved_count invariant must hold");
    assert_eq!(
        budget
            .reserve(super::super::ResourceBudgetAmount::from(usize::MAX))
            .expect_err("e317c775"),
        super::super::ResourceBudgetReserveError::Overflow
    );
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(constants_usize::ONE)
    );
    drop(reservation);
}

#[test]
fn concurrent_reservations_never_exceed_maximum() {
    let budget = super::super::ResourceBudget::new(
        super::super::ResourceBudgetMaximum::try_from(5usize)
            .expect("57a61ca4 concurrent_reservations_never_exceed_maximum invariant must hold"),
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
            let reservation = left_budget.reserve(super::super::ResourceBudgetAmount::from(3usize));
            left_tx.send(reservation.is_ok()).expect(
                "b048535e concurrent_reservations_never_exceed_maximum invariant must hold",
            );
            let _finish_result = left_finish.wait();
            drop(reservation);
        });
        let right_budget = budget.clone();
        let right_start = std::sync::Arc::clone(&start);
        let right_finish = std::sync::Arc::clone(&finish);
        let right_tx = tx.clone();
        let _right_task = scope.spawn(move || {
            let _start_result = right_start.wait();
            let reservation =
                right_budget.reserve(super::super::ResourceBudgetAmount::from(3usize));
            right_tx.send(reservation.is_ok()).expect(
                "cd734995 concurrent_reservations_never_exceed_maximum invariant must hold",
            );
            let _finish_result = right_finish.wait();
            drop(reservation);
        });
        let _start_result = start.wait();
        let outcomes = [
            rx.recv().expect(
                "7393afca concurrent_reservations_never_exceed_maximum invariant must hold",
            ),
            rx.recv().expect(
                "67824b65 concurrent_reservations_never_exceed_maximum invariant must hold",
            ),
        ];
        assert_eq!(
            outcomes.into_iter().filter(|value| *value).count(),
            constants_usize::ONE
        );
        assert_eq!(
            budget.reserved(),
            super::super::ResourceBudgetAmount::from(3usize)
        );
        let _finish_result = finish.wait();
    });
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(constants_usize::ZERO)
    );
}
