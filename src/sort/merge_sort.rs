
// 拆分
fn merge_sort<T: Ord + Copy>(nums: &mut [T])  {
// fn merge_sort(nums: &mut [i32])  {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]);
        merge_sort(&mut nums[mid..]);
        merge(nums, mid);
    }
}

// 合并
fn merge<T: Ord + Copy>(nums: &mut [T], mid: usize) {
// fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0;
    let mut k = mid;
    let mut temp = Vec::new();

    for _j in 0..nums.len() {
        if i == mid || k == nums.len() {
            break;
        }

        // 将数据放到临时集合temp中
        // 比较两个部分，更小的放到temp
        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    // 合并剩余数据，合并的两部分数据大概率长度不一样，因此需要将集合中未处理完的数据全部加入
    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            temp.push(nums[j]);
        }
    }

    // 将temp中的数据放回nums
    for i in 0..nums.len() {
        nums[i] = temp[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
        merge_sort(&mut nums);
        println!("sorted nums: {:?}", nums);
    }

}