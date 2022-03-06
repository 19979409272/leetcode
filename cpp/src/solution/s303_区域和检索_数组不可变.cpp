/*
 * @lc app=leetcode.cn id=303 lang=cpp
 *
 * [303] 区域和检索 - 数组不可变
 */
#include <iostream>
#include <vector>

using namespace std;

// @lc code=start
// 前缀和适用于快速、频繁的计算某个索引区间的和
class NumArray {
public:
  NumArray(vector<int> &nums) {
    // 给前缀和数组分配空间
    length = nums.size() + 1;
    preSum = new int[length];
    // 初始化前缀和数组
    preSum[0] = 0; // 方便运算
    for (int i = 1; i < length; ++i) {
      // preSum[i]: 0-i-1区间的和
      preSum[i] = preSum[i - 1] + nums[i - 1];
    }
  }

  int sumRange(int left, int right) { return preSum[right + 1] - preSum[left]; }

private:
  int *preSum;
  int length;
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * int param_1 = obj->sumRange(left,right);
 */
// @lc code=end
