use std::ops::Sub;

pub fn binary_search(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        // 若 low + high 可能溢出，可以用减法。如果没有溢出风险，直接用 (low + high) >> 1
        let mid = low + ((high - low) >> 1);
        // let mid = (low + high) >> 1;
        // let mid = (low + high) / 2;

        // 必须加上这个条件，否则一直mid等于0的话会无限循环
        if mid == 0 && nums[mid] != num {
            break
        }

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            // num 小于中间值，省去后半部分的数据
            high = mid - 1;
            // high = mid.saturating_sub(1);
        } else {
            low = mid + 1;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let nums = [1,3,8,10,15,32,44,48,50,55,60,62,64];
        let target = 3;
        let found = binary_search(&nums, target);
        println!("found: {:?}", found);

        let target = 63;
        let found = binary_search(&nums, target);
        println!("found: {:?}", found);
    }
}