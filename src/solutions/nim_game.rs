pub fn can_win_nim(n: i32) -> bool  {
    if n % 4 == 0 {
        false
    }
    else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_win_nim() {
        assert_eq!(can_win_nim(4), false);
        assert_eq!(can_win_nim(1), true);
        assert_eq!(can_win_nim(2), true);
    }
}