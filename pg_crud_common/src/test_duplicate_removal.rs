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

#[test]
fn test_hash_duplicate_helpers_distinguish_colliding_keys_and_preserve_first_values() {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        Debug,
        Eq,
        PartialEq,
        proc_macro_newtype_from_inner::FromInner,
    )]
    struct HashCollisionDuplicateKey(crate::duplicate_index::DuplicateIndex);
    impl std::hash::Hash for HashCollisionDuplicateKey {
        fn hash<Hasher>(&self, hasher: &mut Hasher)
        where
            Hasher: std::hash::Hasher,
        {
            hasher.write_u8(0u8);
        }
    }
    let key = |duplicate_index: crate::duplicate_index::DuplicateIndex| {
        HashCollisionDuplicateKey::from(duplicate_index)
    };
    let first = key(crate::duplicate_index::DuplicateIndex::from(0usize));
    let second = key(crate::duplicate_index::DuplicateIndex::from(1usize));
    let third = key(crate::duplicate_index::DuplicateIndex::from(2usize));
    let hash = |hash_collision_duplicate_key: HashCollisionDuplicateKey| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&hash_collision_duplicate_key, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    };
    assert_ne!(first, second);
    assert_ne!(second, third);
    assert_eq!(hash(first), hash(second));
    assert_eq!(hash(second), hash(third));
    assert_eq!(
        crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash(&[
            first, second, third
        ]),
        None
    );
    let mut distinct =
        crate::duplicate_candidates::DuplicateCandidates::from(vec![first, second, third]);
    assert_eq!(
        crate::take_first_duplicate_by_hash::take_first_duplicate_by_hash(&mut distinct),
        None
    );
    assert_eq!(Vec::from(distinct), [first, second, third]);
    let mut repeated = crate::duplicate_candidates::DuplicateCandidates::from(vec![
        first, second, third, second, first,
    ]);
    assert_eq!(
        crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash(repeated.get_inner()),
        Some(crate::duplicate_index::DuplicateIndex::from(3usize))
    );
    assert_eq!(
        crate::take_first_duplicate_by_hash::take_first_duplicate_by_hash(&mut repeated),
        Some(second)
    );
    assert_eq!(Vec::from(repeated), [first, second, third, first]);
    let values = vec![
        (
            second,
            crate::duplicate_index::DuplicateIndex::from(10usize),
        ),
        (third, crate::duplicate_index::DuplicateIndex::from(20usize)),
        (
            second,
            crate::duplicate_index::DuplicateIndex::from(30usize),
        ),
        (first, crate::duplicate_index::DuplicateIndex::from(40usize)),
        (third, crate::duplicate_index::DuplicateIndex::from(50usize)),
    ];
    let expected = [
        (
            second,
            crate::duplicate_index::DuplicateIndex::from(10usize),
        ),
        (third, crate::duplicate_index::DuplicateIndex::from(20usize)),
        (first, crate::duplicate_index::DuplicateIndex::from(40usize)),
    ];
    let unique = crate::deduplicate_preserving_order_by_key::deduplicate_preserving_order_by_key(
        values.into(),
        |value| value.0,
    );
    assert_eq!(Vec::from(unique), expected);
}
