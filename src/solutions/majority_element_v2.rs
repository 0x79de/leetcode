pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut candidate1 = 0;
    let mut candidate2 = 0;
    let mut count1 = 0;
    let mut count2 = 0;

    for num in &nums {
        if count1 > 0 && *num == candidate1 {
            count1 += 1;
        } else if count2 > 0 && *num == candidate2 {
            count2 += 1;
        } else if count1 == 0 {
            candidate1 = *num;
            count1 = 1;
        } else if count2 == 0 {
            candidate2 = *num;
            count2 = 1;
        } else {
            count1 -= 1;
            count2 -= 1;
        }
        
    }
    let threshold = nums.len() / 3;
    let mut result = vec![];
    if count1 > 0 {
        let actual_count1 = nums.iter().filter(|&&x| x == candidate1).count();
        if actual_count1 > threshold {
            result.push(candidate1);
        }
    }
    if count2 > 0 && candidate2 != candidate1 {
        let actual_count2 = nums.iter().filter(|&&x| x == candidate2).count();
        if actual_count2 > threshold {
            result.push(candidate2);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(majority_element(vec![1]), vec![1]);
        assert_eq!(majority_element(vec![1, 2]), vec![1, 2]);
        assert_eq!(majority_element(vec![0]), vec![0]);
    }
}