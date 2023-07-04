
fn nums_sum(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum(&nums[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_sum_test() {
        let nums = [2,1,7,4,5];
        let sum = nums_sum(&nums);
        println!("{:?}", sum);
    }
}