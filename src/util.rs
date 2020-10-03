pub fn group_contiguous_by<T, K: Eq, F: Fn(&T) -> K>(slice: &[T], project: F) -> Vec<(K, &[T])> {
    if slice.is_empty() { return Vec::new() };
    let mut start_ix = 0;
    let mut k_prev = project(&slice[0]);
    let mut result = Vec::new();
    for (this_ix, item) in (&slice[1..]).iter().enumerate() {
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