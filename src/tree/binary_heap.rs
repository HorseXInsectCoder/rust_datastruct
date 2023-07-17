
#[derive(Debug)]
struct BinaryHeap<T> {
    data: Vec<T>,
    size: usize
}

impl<T: Ord + Clone> BinaryHeap<T> {
    fn new() -> Self {
        Self {
            data: vec![],
            size: 0
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 获取堆中最小数据
    fn min(&self) -> Option<T> {
        if 0 == self.size {
            None
        } else {
            Some(self.data[0].clone())
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
        // self.heapify(self.size, (self.data.len() - 1) / 2);
        self.h_sort();
    }

    fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            return None;
        } else if 1 == self.size {
            self.data.pop()
        } else {
            let val = self.data.remove(0);
            self.h_sort();
            self.size -= 1;
            Some(val)
        }
    }

    // fn heapify(&mut self, n: usize, curr: usize) {
    fn heapify(&mut self, n: usize, curr: usize) {
        let mut largest = curr;
        let left_child = 2 * curr + 1;
        let right_child = 2 * curr + 2;

        if left_child < n && self.data[left_child] > self.data[largest] {
            largest = left_child;
        }
        if right_child < n && self.data[right_child] > self.data[largest] {
            largest = right_child;
        }

        if largest != curr {
            self.data.swap(curr, largest);
            self.heapify(n, largest);
        }
    }

    fn h_sort(&mut self) {
        let len = self.data.len();
        if len <= 1 {
            return;
        }

        for i in (0..len / 2).rev() {
            self.heapify(len, i);
        }


        // 第一个元素与最后一个元素交换（使大的元素不断走向堆顶，然后最大的次大的元素不断从数组的最后面往0的方向置换），然后重新调整，直到排序完毕
        for i in (1..len).rev() {
            self.data.swap(0, i);
            self.heapify( i, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_heap_test() {
        let mut bh = BinaryHeap::new();
        // bh.push(3);
        // bh.push(8);
        // bh.push(6);
        // bh.push(1);
        bh.push("aa");
        bh.push("zz");
        bh.push("xx");
        bh.push("bb");
        println!("before pop: {:?}", bh);
        bh.pop();

        println!("{:?}", bh);

        // println!("min: {:?}", bh.min());

    }
}