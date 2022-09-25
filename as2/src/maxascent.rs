pub fn maxascent(a: &Vec<i32>) -> usize {
    if a.is_empty() { return 0; }

    let mut cur = 1usize;
    let mut max = 1usize;

    for i in 1..a.len() {
        if a[i - 1] > a[i] {
            // register
            if cur > max {
                max = cur;
            }
            cur = 1;
        } else {
            cur = cur + 1;
        }
    }

    // register
    if cur > max {
        max = cur;
    }

    max
}

// TODO: max ascent slice를 찾는 것은?

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

    #[test]
    fn test04() {
        assert_eq!(maxascent(&vec![2, 2, 2]), 3);
    }
}
