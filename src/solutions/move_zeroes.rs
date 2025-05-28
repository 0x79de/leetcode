 pub fn move_zeroes(nums: &mut Vec<i32>) {
    let n = nums.len();

    if n == 0 {
        return;
    }

    let mut w_ptr = 0;

    for r_ptr in 0..n {
        if nums[r_ptr] != 0 {
            nums[w_ptr] = nums[r_ptr];
            w_ptr += 1;
        }
    }

    for i in w_ptr..n {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums1 = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums1);
        assert_eq!(nums1, vec![1, 3, 12, 0, 0]);

        let mut nums2 = vec![0];
        move_zeroes(&mut nums2);
        assert_eq!(nums2, vec![0]);
    }
}