use crate::bin_search::binary_search::binary_search;

fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 {
        return false;
    }

    // 逐步找到上界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    // 上界的一半一定可以作为下界
    let low = high >> 1;

    // 使用二分查找
    binary_search(&nums[low..size.min(high + 1 )], target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential_search_test() {
        // let nums = [ 1,9,10,15,16,17,19,23,27,28,29,30,32,35];
        let nums = [1];
        let target = 0;
        let found = exponential_search(&nums, target);
        println!("found: {:?}", found);

        // let nums = [0,1,2,10,16,19,31,25,36,38,40,42,43,55];
        // let nums = [1];
        // let target = 1;
        // let found = exponential_search(&nums, target);
        // println!("found: {:?}", found);
    }
}