
fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut times = 0;

    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high <= low || target < low_val || target > high_val {
            break;
        }

        // 计算插值位置
        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let interpolation = low + offset as usize;
        // times += 1;
        // println!("times: {times}");

        // 更新上/下界 high 和 low
        if nums[interpolation] > target {
            high = interpolation - 1;
        } else if nums[interpolation] < target {
            low = interpolation + 1;
        } else {
            break;
        }
    }

    // 判断最终确实的上界是不是target
    target == nums[high]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolation_search_test() {
        let nums = [1,9,10,15,16,17,19,23,27,28,29,30,32,35];
        let target = 27;
        let found = interpolation_search(&nums, target);
        println!("found: {:?}", found);

        // let nums = [0,1,2,10,16,19,31,25,36,38,40,42,43,55];
        let nums = [0];
        let found = interpolation_search(&nums, target);
        println!("found: {:?}", found);
    }
}