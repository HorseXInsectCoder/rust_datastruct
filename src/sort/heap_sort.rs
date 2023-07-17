

// 返回完全二叉树的数组表示中，一个索引所表示的元素的父亲节点的索引
fn parent(idx: usize) -> usize {
    (idx - 1) / 2
}

// 返回完全二叉树的数组表示中，一个索引所表示的元素的左孩子节点的索引
fn left_child(idx: usize) -> usize {
    (idx << 1) + 1
}

// 返回完全二叉树的数组表示中，一个索引所表示的元素的左孩子节点的索引
fn right_child(idx: usize) -> usize {
    (idx << 1) + 2
}



fn heap_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    // 建堆，从最后一个父节点（非叶子节点）开始，因为新加入元素是从最后面开始加的，新加入的元素破坏了原先的结构，所以要调整堆以满足性质。
    // 下标为 i 的节点的父节点下标：(i-1)/2。最后一个叶子节点的下标为 len - 1，因此最后一个非叶子节点的下标为 (len - 1) / 2。
    // 由于Rust可以直接倒序，所以可以直接写成 (0..len / 2).rev()
    //  (0..len / 2).rev() 构造了一个从最后一个非叶子节点开始的倒序迭代器。对于每个迭代器中的元素 i，我们调用 heapify 函数，将以 i 为根节点的子树调整为最大堆
    // 如果定位最后一个非叶子节点的索引是多少？
    // 答：拿到最后一个节点的索引，然后计算该节点的父节点的索引
    // for i in (0..len / 2).rev() {       // 就是初始起点为 i = parent(arr.length - 1)
    //     heapify(arr, len, i);
    // }
    // for i in (0..len >> 1).rev() {
    //     heapify(arr, len, i);
    // }

    for i in (0..=parent(len - 1)).rev() {
        heapify(arr, len, i);
    }


    // 第一个元素与最后一个元素交换（使大的元素不断走向堆顶，然后最大的次大的元素不断从数组的最后面往0的方向置换），然后重新调整，直到排序完毕
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

// 将任意数组整理成堆的形状。从最后一个非叶子开始开始
// i 是当前节点
fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    // let left = 2 * i + 1;
    // let right = 2 * i + 2;
    let left = left_child(i);
    let right = right_child(i);

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        // 因为交换后可能导致子节点不满足二叉堆性质，所以要递归调用
        heapify(arr, n, largest);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_heap_test() {
        let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
        heap_sort(&mut nums);
        println!("{:?}", nums);
    }
}