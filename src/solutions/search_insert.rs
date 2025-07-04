pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(index) | Err(index) => index as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    } 
}
