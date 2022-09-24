pub fn is_palindrome(input: &str) -> bool {
    false
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
}
