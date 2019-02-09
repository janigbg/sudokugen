/// Group represents a row, column or box
pub type Group = [u8; 9];

///
pub fn add(a: Group, b: &Group) -> Group {
    new_array_from(a.iter().zip(b).map(|(a, b)| a + b))
}

pub fn sub(a: Group, b: &Group) -> Group {
    new_array_from(a.iter().zip(b).map(|(a, b)| a - b))
}

pub fn new_array_from<F: Iterator<Item = u8>>(src: F) -> Group {
    let mut result = [0; 9];
    for (result_ref, val) in result.iter_mut().zip(src) {
        *result_ref = val;
    }
    result
}
