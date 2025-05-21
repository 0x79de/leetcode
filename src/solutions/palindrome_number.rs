pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut rev_half = 0;

    while rev_half < x {
        rev_half = rev_half * 10 + (x % 10);
        x /= 10;
    }

    x == rev_half || x == rev_half / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(12321), true);
        assert_eq!(is_palindrome(0), true);
    }
}
