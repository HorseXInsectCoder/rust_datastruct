
fn sequential_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;

    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true
        } else {
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
    fn sequential_search_test() {
        let num = 8;
        let nums = [9, 3, 7, 4, 1, 6, 2, 8, 5];
        match sequential_search(&nums, num) {
            Some(pos) => println!("{}", pos),
            None => println!("not exist")
        }
    }
}