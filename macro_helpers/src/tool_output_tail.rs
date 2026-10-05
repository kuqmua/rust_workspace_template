#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ToolOutputTail {
    bytes: crate::tool_output_byte_vec_deque::ToolOutputByteVecDeque,
    limit: crate::tool_output_limit::ToolOutputLimit,
}

impl ToolOutputTail {
    pub(crate) fn new(tool_output_limit: crate::tool_output_limit::ToolOutputLimit) -> Self {
        Self {
            bytes: crate::tool_output_byte_vec_deque::ToolOutputByteVecDeque::default(),
            limit: tool_output_limit,
        }
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        std::collections::VecDeque::<crate::tool_output_byte::ToolOutputByte>::from(self.bytes)
            .into_iter()
            .map(|byte| *byte.get_inner())
            .collect()
    }
}

impl std::io::Write for ToolOutputTail {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let retained = bytes.len().min(*self.limit.get_inner());
        if retained == *self.limit.get_inner() {
            self.bytes.clear();
        } else {
            let removed = self
                .bytes
                .len()
                .saturating_add(retained)
                .saturating_sub(*self.limit.get_inner());
            let _removed_bytes = self.bytes.drain(..removed).count();
        }
        self.bytes.extend(
            bytes
                .iter()
                .skip(bytes.len().saturating_sub(retained))
                .copied()
                .map(crate::tool_output_byte::ToolOutputByte::from),
        );
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_output_tail_retains_latest_bytes_across_writes() {
        let mut tail =
            super::ToolOutputTail::new(crate::tool_output_limit::ToolOutputLimit::from(5usize));
        assert!(matches!(
            std::io::Write::write_all(&mut tail, &[1u8, 2u8, 3u8, 4u8]),
            Ok(())
        ));
        assert!(matches!(
            std::io::Write::write_all(&mut tail, &[5u8, 6u8, 7u8]),
            Ok(())
        ));
        assert_eq!(tail.into_bytes(), vec![3u8, 4u8, 5u8, 6u8, 7u8]);
    }

    #[test]
    fn test_output_tail_handles_single_oversized_write() {
        let mut tail =
            super::ToolOutputTail::new(crate::tool_output_limit::ToolOutputLimit::from(3usize));
        assert!(matches!(
            std::io::Write::write_all(&mut tail, &[1u8, 2u8, 3u8, 4u8, 5u8]),
            Ok(())
        ));
        assert_eq!(tail.into_bytes(), vec![3u8, 4u8, 5u8]);
    }

    #[test]
    fn test_output_tail_empty_writes_and_flush_preserve_bounded_content() {
        [
            (0usize, Vec::new()),
            (1usize, vec![3u8]),
            (3usize, vec![1u8, 2u8, 3u8]),
        ]
        .into_iter()
        .fold((), |(), (limit, expected)| {
            let mut tail =
                super::ToolOutputTail::new(crate::tool_output_limit::ToolOutputLimit::from(limit));
            assert!(matches!(
                std::io::Write::write(&mut tail, &[1u8, 2u8, 3u8]),
                Ok(3usize)
            ));
            assert!(matches!(std::io::Write::write(&mut tail, &[]), Ok(0usize)));
            assert!(matches!(std::io::Write::flush(&mut tail), Ok(())));
            assert_eq!(tail.into_bytes(), expected);
        });
    }
}
