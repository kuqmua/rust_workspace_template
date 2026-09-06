#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_new::New)]
pub(crate) struct FrontendDependencyInputs {
    package_manifest: crate::bounded_text::BoundedText,
    package_lock: crate::bounded_text::BoundedText,
    node_version: crate::bounded_text::BoundedText,
}

impl FrontendDependencyInputs {
    #[allow(
        clippy::little_endian_bytes,
        reason = "the local dependency stamp serializes its fixed-width hash in a consistent byte order"
    )]
    pub(crate) fn fingerprint(
        &self,
    ) -> crate::frontend_dependency_fingerprint::FrontendDependencyFingerprint {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(self.package_manifest.as_ref(), &mut hasher);
        std::hash::Hash::hash(self.package_lock.as_ref(), &mut hasher);
        std::hash::Hash::hash(self.node_version.as_ref(), &mut hasher);
        crate::frontend_dependency_fingerprint::FrontendDependencyFingerprint::from(
            std::hash::Hasher::finish(&hasher).to_le_bytes(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(
        clippy::panic_in_result_fn,
        clippy::needless_for_each,
        reason = "the test harness propagates fallible bounded fixture setup and asserts every cache input independently without forbidden for loops"
    )]
    fn test_frontend_preparation_dependency_cache_tracks_all_inputs()
    -> Result<(), crate::bounded_read_error::BoundedReadError> {
        let fixture = || {
            crate::bounded_text::BoundedText::try_from(
                constants_str::FRONTEND_DEPENDENCY_FIXTURE_ONE.to_owned(),
            )
        };
        let changed =
            || crate::bounded_text::BoundedText::try_from(constants_str::VALUE_4F53CDA1.to_owned());
        let baseline = crate::frontend_dependency_inputs::FrontendDependencyInputs::new(
            fixture()?,
            fixture()?,
            fixture()?,
        );
        assert_eq!(baseline.fingerprint(), baseline.fingerprint());
        [
            crate::frontend_dependency_inputs::FrontendDependencyInputs::new(
                changed()?,
                fixture()?,
                fixture()?,
            ),
            crate::frontend_dependency_inputs::FrontendDependencyInputs::new(
                fixture()?,
                changed()?,
                fixture()?,
            ),
            crate::frontend_dependency_inputs::FrontendDependencyInputs::new(
                fixture()?,
                fixture()?,
                changed()?,
            ),
        ]
        .into_iter()
        .for_each(|inputs| assert_ne!(baseline.fingerprint(), inputs.fingerprint()));
        Ok(())
    }
}
