pub fn maxascent(a: &Vec<i32>) -> usize {
    a.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(maxascent(&vec![]), 0);
        assert_eq!(maxascent(&vec![1]), 1);
        assert_eq!(maxascent(&vec![1, 2]), 2);
    }

    #[test]
    fn test02() {
        assert_eq!(maxascent(&vec![1, 2, 1]), 2);
        assert_eq!(maxascent(&vec![1, 2, 1, 2]), 2);
        assert_eq!(maxascent(&vec![4, 3, 2, 1]), 1);
        assert_eq!(maxascent(&vec![4, 3, 2, 1, 2, 1]), 2);
    }

    #[test]
    fn test03() {
        assert_eq!(maxascent(&vec![2, 3, 4, 3, 2, 1, 2, 1]), 3);
        assert_eq!(maxascent(&vec![2, 3, 4, 3, 2, 1, 2, 1, 2, 3, 4]), 4);
    }
}
