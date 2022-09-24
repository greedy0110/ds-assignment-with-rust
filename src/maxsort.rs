use crate::arraryutility::*;

fn max_sort_shift(v: &mut Vec<i32>) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sort() {
        let mut v = vec![1, 2, 3, 4, 5];
        max_sort_shift(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }
}
