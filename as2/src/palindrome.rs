pub fn is_palindrome(input: &str) -> bool {
    // input이 오직 ascii 코드로만 이루어져 각 요소가 1byte라고 가정.
    assert!(input.chars().count() == input.len());

    fn _is_palindrome(input: &str, i: usize, j:usize) -> bool {
        let len = j - i;
        if len == 0 || len == 1 {
            return true;
        }
        let bytes = input.as_bytes();
        bytes[i] == bytes[j-1] && _is_palindrome(input, i+1, j-1)
    }
    _is_palindrome(input, 0, input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true0() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("aa"));
        assert!(is_palindrome("aaa"));
        assert!(is_palindrome("radar"));
        assert!(is_palindrome("rats live on no evil star"));
    }

    #[test]
    fn false0() {
        assert!(!is_palindrome("abcab"));
        assert!(!is_palindrome("ab a"));
    }

    #[test]
    #[should_panic]
    fn painc_non_ascii() {
        assert!(!is_palindrome("하"));
        assert!(!is_palindrome("👍"));
    }
}
