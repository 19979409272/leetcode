/*
 * @lc app=leetcode.cn id=303 lang=rust
 *
 * [303] 区域和检索 - 数组不可变
 */

// @lc code=start
struct NumArray {
    pre_sum: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(mut nums: Vec<i32>) -> Self {
        for i in 1..nums.len() {
            // nums[i]: 索引i的前缀和（包括索引i的值）
            nums[i] += nums[i - 1];
        }
        Self {
            pre_sum: nums,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.pre_sum[right as usize]
        } else {
            self.pre_sum[right as usize] - self.pre_sum[left as usize - 1]
        }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
// @lc code=end

