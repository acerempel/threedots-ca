/// Divide a slice into groups, where each group is a contiguous subslice of the input slice
/// whose elements have some property in common, as determined by the provided function.
pub fn group_contiguous_by<T, K: Eq, F: Fn(&T) -> K>(slice: &[T], project: F) -> Vec<(K, &[T])> {
    if slice.is_empty() { return Vec::new() };
    let mut start_ix = 0;
    let mut k_prev = project(&slice[0]);
    let mut result = Vec::new();
    // [Note 1]: `enumerate()` starts the indexes at zero from the beginning of the iterator, but
    // we are interested really in indexes into the original slice, so we have to add one to
    // account for the fact that we took a subslice omitting the first element.
    for (this_ix_1, item) in (&slice[1..]).iter().enumerate() {
        let this_ix = this_ix_1 + 1; // See note 1
        let k_cur = project(item);
        if k_cur != k_prev {
            result.push((k_prev, &slice[start_ix..this_ix]));
            start_ix = this_ix;
            k_prev = k_cur;
        }
    }
    result.push((k_prev, &slice[start_ix..slice.len()]));
    result
}
