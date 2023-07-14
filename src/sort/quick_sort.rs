use std::borrow::{Borrow, BorrowMut};

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot = partition2(arr, 0, arr.len() - 1);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }
}

// fn partition<T: Ord>(arr: &mut [T]) -> usize {
//     let pivot_index = arr.len() - 1;
//     let mut i = 0;
//     for j in 0..arr.len() - 1 {
//         if arr[j] < arr[pivot_index] {
//             arr.swap(i, j);
//             i += 1;
//         }
//     }
//     arr.swap(i, pivot_index);
//     i
// }

fn partition2<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut lm = low;
    let mut rm = high;

    loop {
        while lm <= rm && arr[lm] <= arr[low] {
            lm += 1;
        }
        while lm <= rm && arr[rm] >= arr[low] {
            rm -= 1;
        }
        if lm > rm {
            break;
        }
        arr.swap(lm, rm);
    }
    arr.swap(low, rm);
    rm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
        quick_sort(&mut nums);
        println!("sorted nums: {:?}", nums);
    }

}