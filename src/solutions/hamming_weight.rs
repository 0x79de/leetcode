pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;

    while n != 0 {
        count += n  & 1;
        n >>= 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(11), 3);
        assert_eq!(hamming_weight(128), 1);
        assert_eq!(hamming_weight(2147483645), 30);
    }
}