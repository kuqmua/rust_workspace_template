#[test]
fn test_duplicate_search_and_removal_agree_for_all_small_sequences() {
    let sequences = (0..=5usize).flat_map(|length| {
        (0..length).fold(vec![Vec::<u8>::new()], |prefixes, _| {
            prefixes
                .into_iter()
                .flat_map(|prefix| {
                    (0..3u8).map(move |value| {
                        let mut sequence = prefix.clone();
                        sequence.push(value);
                        sequence
                    })
                })
                .collect::<Vec<_>>()
        })
    });
    assert!(sequences.into_iter().all(|sequence| {
        let expected_index = sequence.iter().enumerate().find_map(|(index, value)| {
            (sequence
                .iter()
                .take(index)
                .filter(|prior| *prior == value)
                .count()
                > 0)
            .then_some(index)
        });
        let expected_duplicate_index =
            expected_index.map(crate::duplicate_index::DuplicateIndex::from);
        assert_eq!(
            crate::first_duplicate_index::first_duplicate_index(&sequence),
            expected_duplicate_index
        );
        assert_eq!(
            crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash(&sequence),
            expected_duplicate_index
        );
        let mut expected_sequence = sequence.clone();
        let expected_removed = expected_index.map(|index| expected_sequence.swap_remove(index));
        let mut equality_candidates =
            crate::duplicate_candidates::DuplicateCandidates::from(sequence.clone());
        let mut hash_candidates = crate::duplicate_candidates::DuplicateCandidates::from(sequence);
        assert_eq!(
            crate::take_first_duplicate::take_first_duplicate(&mut equality_candidates),
            expected_removed
        );
        assert_eq!(
            crate::take_first_duplicate_by_hash::take_first_duplicate_by_hash(&mut hash_candidates),
            expected_removed
        );
        Vec::from(equality_candidates) == expected_sequence
            && Vec::from(hash_candidates) == expected_sequence
    }));
}
