/// Group represents values (e.g. counters) for positions a row,
/// column or box.
pub type Group = [u8; 9];

/// Add two groups (vector element addition), producing a new group.
pub fn add(a: Group, b: &Group) -> Group {
    new_array_from(a.iter().zip(b).map(|(a, b)| a + b))
}

/// Subtract one group from another (vector element subtraction),
/// producing a new group.
pub fn sub(a: Group, b: &Group) -> Group {
    new_array_from(a.iter().zip(b).map(|(a, b)| a - b))
}

/// Produce new group from iterator of `u8`.
pub fn new_array_from<F: Iterator<Item = u8>>(src: F) -> Group {
    let mut result = [0; 9];
    for (result_ref, val) in result.iter_mut().zip(src) {
        *result_ref = val;
    }
    result
}
