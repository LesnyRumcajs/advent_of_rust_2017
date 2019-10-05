/// Finds the first element that is different from others in a container.
/// Returns none if all elements are equal or the container is ambiguous e.g.
/// less than 3 elements.
pub fn first_different_element<T>(v: &[T]) -> Option<usize>
where
    T: Eq,
{
    if v.len() < 3 {
        return None;
    }

    if v[0] == v[1] && v[0] != v[2] {
        return Some(2);
    }

    if v[0] == v[2] && v[0] != v[1] {
        return Some(1);
    }

    if v[1] == v[2] && v[0] != v[1] {
        return Some(0);
    }

    for (i, el) in v.iter().enumerate().skip(3) {
        if el != &v[i - 1] {
            return Some(i);
        }
    }

    None
}

#[test]
fn different_elements() {
    let v = vec![1, 1, 1, 3, 1];
    assert_eq!(first_different_element(&v), Some(3));

    let v: Vec<String> = Vec::new();
    assert_eq!(first_different_element(&v), None);

    let v: Vec<i32> = vec![3, 1, 1];
    assert_eq!(first_different_element(&v), Some(0));

    let v: Vec<i32> = vec![1, 3, 1];
    assert_eq!(first_different_element(&v), Some(1));

    let v: Vec<i32> = vec![1, 1, 3];
    assert_eq!(first_different_element(&v), Some(2));

    let v: Vec<i32> = vec![1, 1, 1];
    assert_eq!(first_different_element(&v), None);
}
