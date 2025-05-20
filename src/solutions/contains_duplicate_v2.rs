use std::collections::HashSet;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut seen = HashSet::new();
    let k_val = k as usize;

    for (i ,num_ref) in nums.iter().enumerate() {
        let n = *num_ref;
        if seen.contains(&n) {
            return true;
        }
        seen.insert(n);
        if seen.len() > k_val {
            seen.remove(&nums[i - k_val]);
        }
    }
    false

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_nearby_duplicate(vec![1,2,3,1], 3), true);
        assert_eq!(contains_nearby_duplicate(vec![1,0,1,1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1,2,3,1,2,3], 2), false);
    }
}   

