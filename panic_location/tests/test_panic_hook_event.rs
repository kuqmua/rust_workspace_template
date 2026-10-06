#![allow(
    unused_crate_dependencies,
    reason = "integration target exercises the panic hook; remaining dependencies implement library domain wrappers"
)]

#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_from_inner::FromInner,
        proc_macro_newtype_get_inner::GetInner,
    )]
    #[borrow]
    struct TestPanicStdAtomicBool(std::sync::atomic::AtomicBool);

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_from_inner::FromInner,
        proc_macro_newtype_get_inner::GetInner,
    )]
    #[borrow]
    struct TestPanicEventSubscriber(TestPanicStdAtomicBool);

    impl tracing::Subscriber for TestPanicEventSubscriber {
        fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
            *metadata.level() == tracing::Level::ERROR
        }

        fn enter(&self, id: &tracing::span::Id) {
            tracing::Subscriber::enter(&tracing::subscriber::NoSubscriber::default(), id);
        }

        fn event(&self, event: &tracing::Event<'_>) {
            let debug = format!("{event:?}");
            self.get().get().store(
                *event.metadata().level() == tracing::Level::ERROR
                    && debug.contains(file!())
                    && debug.contains(constants_str::TRACING_PANIC_CAPTURED)
                    && !debug.contains(constants_str::PANIC_HOOK_EVENT_TEST_PAYLOAD),
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        fn exit(&self, id: &tracing::span::Id) {
            tracing::Subscriber::exit(&tracing::subscriber::NoSubscriber::default(), id);
        }

        fn new_span(&self, attributes: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::Subscriber::new_span(&tracing::subscriber::NoSubscriber::default(), attributes)
        }

        fn record(&self, id: &tracing::span::Id, record: &tracing::span::Record<'_>) {
            tracing::Subscriber::record(&tracing::subscriber::NoSubscriber::default(), id, record);
        }

        fn record_follows_from(&self, id: &tracing::span::Id, follows: &tracing::span::Id) {
            tracing::Subscriber::record_follows_from(
                &tracing::subscriber::NoSubscriber::default(),
                id,
                follows,
            );
        }
    }

    #[test]
    fn test_panic_hook_emits_location_event_without_panic_payload() {
        let dispatch = tracing::Dispatch::new(TestPanicEventSubscriber::from(
            TestPanicStdAtomicBool::from(std::sync::atomic::AtomicBool::from(false)),
        ));
        let caught = tracing::dispatcher::with_default(&dispatch, || {
            panic_location::panic_location();
            panic_location::panic_location();
            std::panic::catch_unwind(|| {
                std::panic::panic_any(constants_str::PANIC_HOOK_EVENT_TEST_PAYLOAD);
            })
        });
        assert!(caught.is_err_and(|payload| {
            payload.downcast_ref::<&str>() == Some(&constants_str::PANIC_HOOK_EVENT_TEST_PAYLOAD)
        }));
        assert!(
            dispatch
                .downcast_ref::<TestPanicEventSubscriber>()
                .is_some_and(|subscriber| {
                    subscriber
                        .get()
                        .get()
                        .load(std::sync::atomic::Ordering::Relaxed)
                })
        );
    }
}
