#[test]
fn maximum_rejects_zero_and_accepts_positive_values() {
    assert_eq!(
        super::super::ResourceBudgetMaximum::try_from(0usize),
        Err(super::super::ResourceBudgetConfigError)
    );
    let maximum = std::num::NonZeroUsize::new(1usize).expect("9e83081e");
    assert_eq!(
        super::super::ResourceBudgetMaximum::try_from(1usize).expect("19c82820"),
        super::super::ResourceBudgetMaximum::from(maximum)
    );
}

#[test]
fn reservations_are_bounded_and_released() {
    let budget = super::super::ResourceBudget::new(
        super::super::ResourceBudgetMaximum::try_from(5usize).expect("0c6362a4"),
    );
    let first = budget
        .reserve(super::super::ResourceBudgetAmount::from(3usize))
        .expect("3bfeb37c");
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
        .expect("d86085db");
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
        super::super::ResourceBudgetAmount::from(0usize)
    );
}

#[test]
fn overflow_does_not_change_reserved_count() {
    let budget = super::super::ResourceBudget::new(
        super::super::ResourceBudgetMaximum::try_from(usize::MAX).expect("65f2f229"),
    );
    let reservation = budget
        .reserve(super::super::ResourceBudgetAmount::from(1usize))
        .expect("1a2bb321");
    assert_eq!(
        budget
            .reserve(super::super::ResourceBudgetAmount::from(usize::MAX))
            .expect_err("e317c775"),
        super::super::ResourceBudgetReserveError::Overflow
    );
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(1usize)
    );
    drop(reservation);
}

#[test]
fn concurrent_reservations_never_exceed_maximum() {
    let budget = super::super::ResourceBudget::new(
        super::super::ResourceBudgetMaximum::try_from(5usize).expect("57a61ca4"),
    );
    let start = std::sync::Arc::new(std::sync::Barrier::new(3usize));
    let finish = std::sync::Arc::new(std::sync::Barrier::new(3usize));
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::scope(|scope| {
        let left_budget = budget.clone();
        let left_start = std::sync::Arc::clone(&start);
        let left_finish = std::sync::Arc::clone(&finish);
        let left_tx = tx.clone();
        let _left_handle = scope.spawn(move || {
            let _start_result = left_start.wait();
            let reservation = left_budget.reserve(super::super::ResourceBudgetAmount::from(3usize));
            left_tx.send(reservation.is_ok()).expect("b048535e");
            let _finish_result = left_finish.wait();
            drop(reservation);
        });
        let right_budget = budget.clone();
        let right_start = std::sync::Arc::clone(&start);
        let right_finish = std::sync::Arc::clone(&finish);
        let right_tx = tx.clone();
        let _right_handle = scope.spawn(move || {
            let _start_result = right_start.wait();
            let reservation =
                right_budget.reserve(super::super::ResourceBudgetAmount::from(3usize));
            right_tx.send(reservation.is_ok()).expect("cd734995");
            let _finish_result = right_finish.wait();
            drop(reservation);
        });
        let _start_result = start.wait();
        let outcomes = [rx.recv().expect("7393afca"), rx.recv().expect("67824b65")];
        assert_eq!(outcomes.into_iter().filter(|value| *value).count(), 1usize);
        assert_eq!(
            budget.reserved(),
            super::super::ResourceBudgetAmount::from(3usize)
        );
        let _finish_result = finish.wait();
    });
    assert_eq!(
        budget.reserved(),
        super::super::ResourceBudgetAmount::from(0usize)
    );
}
