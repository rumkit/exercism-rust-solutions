pub fn find<T1,T2>(array: T1, key: T2) -> Option<usize>
where T1: AsRef<[T2]>, T2: PartialOrd {
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if array[mid] < key {
            left = mid + 1;
        }
        else if array[mid] > key {
            right = mid;
        }
        else {
            return Some(mid);
        }
    }

    None
}
