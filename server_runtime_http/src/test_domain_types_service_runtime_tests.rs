#[cfg(test)]
mod tests {
    #[test]
    fn test_service_runtime_builder_enables_tokio_runtime() {
        let wrapped_runtime = crate::build_service_runtime::build_service_runtime()
            .expect(constants_str::DIAGNOSTIC_5ECC3726);
        let runtime = tokio::runtime::Runtime::from(wrapped_runtime);
        assert_eq!(runtime.block_on(async { 2u8 + 2u8 }), 4u8);
    }
}
