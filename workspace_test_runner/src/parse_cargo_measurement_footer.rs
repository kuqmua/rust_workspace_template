pub(crate) fn parse_cargo_measurement_footer(
    stderr_text_ref: crate::stderr_text_ref::StderrTextRef<'_>,
) -> Option<crate::cargo_measurement_footer::CargoMeasurementFooter<'_>> {
    let text = stderr_text_ref.get().trim_end_matches('\n');
    let (before_major, major_line) = text.rsplit_once('\n')?;
    let major_page_faults =
        major_line.strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)?;
    let (before_minor, minor_line) = before_major.rsplit_once('\n')?;
    let minor_page_faults =
        minor_line.strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)?;
    let (program_text, peak_rss_kb) =
        before_minor.rsplit_once(constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)?;
    if [peak_rss_kb, minor_page_faults, major_page_faults]
        .into_iter()
        .any(|value| value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }
    Some(
        crate::cargo_measurement_footer::CargoMeasurementFooter::new(
            crate::stderr_text_ref::StderrTextRef::from(major_page_faults),
            crate::stderr_text_ref::StderrTextRef::from(minor_page_faults),
            crate::stderr_text_ref::StderrTextRef::from(peak_rss_kb),
            crate::stderr_text_ref::StderrTextRef::from(program_text),
        ),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_footer_uses_final_markers_and_preserves_program_lines() {
        let fake_peak = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::VALUE_1
        );
        let fake_minor = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_1
        );
        let fake_major = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_1
        );
        let real_peak = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::VALUE_2
        );
        let real_minor = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_4
        );
        let real_major = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_2
        );
        let stderr = [
            fake_peak.as_str(),
            fake_minor.as_str(),
            fake_major.as_str(),
            real_peak.as_str(),
            real_minor.as_str(),
            real_major.as_str(),
        ]
        .join(constants_str::NEWLINE);
        let maybe_footer = super::parse_cargo_measurement_footer(
            crate::stderr_text_ref::StderrTextRef::from(stderr.as_str()),
        );
        assert!(maybe_footer.is_some_and(|footer| {
            footer.get_peak_rss_kb().get() == constants_str::VALUE_2
                && footer.get_minor_page_faults().get() == constants_str::VALUE_4
                && footer.get_major_page_faults().get() == constants_str::VALUE_2
                && footer.get_program_text().get().lines().eq([
                    fake_peak.as_str(),
                    fake_minor.as_str(),
                    fake_major.as_str(),
                ])
        }));
    }

    #[test]
    fn test_footer_keeps_program_text_without_trailing_newline() {
        let peak = format!(
            "{}{}{}",
            constants_str::X,
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::VALUE_2
        );
        let minor = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_4
        );
        let major = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_1
        );
        let stderr = [peak.as_str(), minor.as_str(), major.as_str()].join(constants_str::NEWLINE);
        let maybe_footer = super::parse_cargo_measurement_footer(
            crate::stderr_text_ref::StderrTextRef::from(stderr.as_str()),
        );
        assert!(maybe_footer.is_some_and(|footer| {
            footer.get_program_text().get() == constants_str::X
                && footer.get_peak_rss_kb().get() == constants_str::VALUE_2
                && footer.get_minor_page_faults().get() == constants_str::VALUE_4
                && footer.get_major_page_faults().get() == constants_str::VALUE_1
        }));
    }

    #[test]
    fn test_malformed_footer_is_not_consumed() {
        let peak = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::VALUE_1
        );
        let minor = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_2
        );
        let major = format!(
            "{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
            constants_str::X
        );
        let stderr = [peak.as_str(), minor.as_str(), major.as_str()].join(constants_str::NEWLINE);
        assert!(
            super::parse_cargo_measurement_footer(crate::stderr_text_ref::StderrTextRef::from(
                stderr.as_str()
            ))
            .is_none()
        );
    }
    #[test]
    fn test_footer_rejects_invalid_values_in_every_measurement_field() {
        [
            constants_str::EMPTY,
            constants_str::X,
            constants_str::VALUE_F1234D75,
            constants_str::SPACE,
        ]
        .into_iter()
        .fold((), |(), invalid| {
            [
                [invalid, constants_str::VALUE_1, constants_str::VALUE_1],
                [constants_str::VALUE_1, invalid, constants_str::VALUE_1],
                [constants_str::VALUE_1, constants_str::VALUE_1, invalid],
            ]
            .into_iter()
            .fold((), |(), values| {
                let stderr = format!(
                    "{}{}\n{}{}\n{}{}",
                    constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
                    values[constants_usize::ZERO],
                    constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
                    values[constants_usize::ONE],
                    constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
                    values[constants_usize::TWO],
                );
                assert!(
                    super::parse_cargo_measurement_footer(
                        crate::stderr_text_ref::StderrTextRef::from(stderr.as_str())
                    )
                    .is_none()
                );
            });
        });
    }

    #[test]
    fn test_footer_accepts_zero_measurements_and_trailing_newlines() {
        let stderr = format!(
            "{}{}\n{}{}\n{}{}\n\n",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::VALUE_0,
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_0,
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
            constants_str::VALUE_0,
        );
        assert!(
            super::parse_cargo_measurement_footer(crate::stderr_text_ref::StderrTextRef::from(
                stderr.as_str()
            ))
            .is_some_and(|footer| {
                footer.get_program_text().get().is_empty()
                    && footer.get_peak_rss_kb().get() == constants_str::VALUE_0
                    && footer.get_minor_page_faults().get() == constants_str::VALUE_0
                    && footer.get_major_page_faults().get() == constants_str::VALUE_0
            })
        );
    }

    #[test]
    fn test_footer_requires_all_markers_in_order_at_the_end() {
        let lines = [
            format!(
                "{}{}",
                constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
                constants_str::VALUE_1
            ),
            format!(
                "{}{}",
                constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
                constants_str::VALUE_1
            ),
            format!(
                "{}{}",
                constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
                constants_str::VALUE_1
            ),
        ];
        [
            vec![],
            vec![lines[constants_usize::ZERO].as_str()],
            vec![
                lines[constants_usize::ZERO].as_str(),
                lines[constants_usize::ONE].as_str(),
            ],
            vec![
                lines[constants_usize::ONE].as_str(),
                lines[constants_usize::ZERO].as_str(),
                lines[constants_usize::TWO].as_str(),
            ],
            vec![
                lines[constants_usize::ZERO].as_str(),
                lines[constants_usize::ONE].as_str(),
                lines[constants_usize::TWO].as_str(),
                constants_str::X,
            ],
        ]
        .into_iter()
        .fold((), |(), malformed| {
            let stderr = malformed.join(constants_str::NEWLINE);
            assert!(
                super::parse_cargo_measurement_footer(crate::stderr_text_ref::StderrTextRef::from(
                    stderr.as_str()
                ))
                .is_none()
            );
        });
    }
}
