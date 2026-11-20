use alloc::vec::Vec;

/// PRUNING: drops collected `Ok` values and ignores later `Ok` values after the first `Err`, because `handle_iter!` only returns errors when any item fails.
#[doc(hidden)]
pub fn partition_result<T, E>(results: impl IntoIterator<Item = Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let iter = results.into_iter();
    let (lower, _) = iter.size_hint();
    let (oks, errors) = iter.fold((Vec::with_capacity(lower), Vec::new()), |(mut oks, mut errors), result| {
        match result {
            Ok(value) => {
                if errors.is_empty() {
                    oks.push(value);
                }
            }
            Err(error) => {
                if errors.is_empty() {
                    oks = Vec::new();
                }
                errors.push(error);
            }
        }
        (oks, errors)
    });

    if errors.is_empty() { Ok(oks) } else { Err(errors) }
}
