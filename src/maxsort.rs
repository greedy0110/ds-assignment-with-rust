use crate::arraryutility::*;

fn max_sort_shift(v: &mut Vec<i32>) {
    let end = v.len();
    for left in 0..end {
        let i_max = findmaxpos(&v, left, end);
        let max = v[i_max];
        shift_right(v, left, i_max + 1);
        v[left] = max;
    }
}

fn max_sort_swap(v: &mut Vec<i32>) {
    let end = v.len();
    for left in 0..end {
        let i_max = findmaxpos(&v, left, end);
        if left != i_max { swap(v, left, i_max); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sort_shift() {
        let mut v = vec![1, 2, 3, 4, 5];
        max_sort_shift(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_max_sort_swap() {
        let mut v = vec![1, 2, 3, 4, 5];
        max_sort_swap(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }
}
