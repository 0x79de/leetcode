pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut unique_count = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[unique_count - 1] {
            nums[unique_count] = nums[i];
            unique_count += 1;
        }
    }
    nums.truncate(unique_count);
    unique_count as i32
            
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }
}
