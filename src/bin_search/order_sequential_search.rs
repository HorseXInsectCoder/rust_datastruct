
fn order_sequential_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false;

    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true
        } else if num < nums[pos] {
            stop = true;            // 数据升序的情况下，遇到比目标要大的数，说明后面也不会找到了，退出
        }
        else {
            pos += 1;
        }
    }
    if found {
        Some(pos)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_sequential_search_test() {
        let num = 44;
        let nums = [1,3,8,10,15,32,44,48,50,55,60,62,64];
        match order_sequential_search(&nums, num) {
            Some(pos) => println!("{}", pos),
            None => println!("not exist")
        }

        let num = 49;
        match order_sequential_search(&nums, num) {
            Some(pos) => println!("{}", pos),
            None => println!("not exist")
        }
    }
}