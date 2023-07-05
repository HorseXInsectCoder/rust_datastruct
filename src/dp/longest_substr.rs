use std::cmp::max;
use std::collections::HashMap;

// 返回从第 i 个数字开始的最长子序列
fn longest_str_v1(nums: &[i32], i: u32) -> u32 {
    if i == (nums.len() - 1) as u32{
        return 1;
    }

    let mut max_len = 1;
    for j in (i+1)..nums.len() as u32 {
        if nums[j as usize] > nums[i as usize] {
            max_len = max(max_len, longest_str_v1(nums, j) + 1)
        }
    }

    max_len
}

// 用 hash 保存已经搜索过的结果
fn longest_str_v2(nums: &[i32], i: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if i == (nums.len() - 1) as u32{
        return 1;
    }

    // 检查是否保存过 memo，如果保存过，就立即返回结果
    if memo.contains_key(&i) {
        return memo[&i]
    }

    let mut max_len = 1;
    for j in (i+1)..nums.len() as u32 {
        if nums[j as usize] > nums[i as usize] {
            max_len = max(max_len, longest_str_v2(nums, j, memo) + 1)
        }
    }

    memo.insert(i, max_len);

    max_len
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn longest_str_v1_test() {
        let start = Instant::now();
        let nums = [1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88, 1,5,2,4,3, 22
            , 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88,
            1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88,
            1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88,
            1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88];
        let res = longest_str_v1(&nums, 0);
        println!("res: {:?}", res);
        let end = Instant::now();
        let duration = end - start;
        println!("程序运行时间为：{:?}", duration);
    }

    #[test]
    fn longest_str_v2_test() {
        let start = Instant::now();
        // let nums = [1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88, 1,5,2,4,3, 22
        //     , 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88,
        //     1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88,
        //     1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88,
        //     1,5,2,4,3, 22, 8, 110, 100, 555, 666, 888, 678, 123, 101, 86, 59, 32, 11, 21, 31, 90, 95, 88];
        let nums = [1,5,2,4,3];
        let mut memo: HashMap<u32, u32> = HashMap::new();
        let res = longest_str_v2(&nums, 0, &mut memo);
        println!("res: {:?}", res);
        let end = Instant::now();
        let duration = end - start;
        println!("程序运行时间为：{:?}", duration);
    }
}
