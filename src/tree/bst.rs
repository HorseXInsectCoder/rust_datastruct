use std::cmp::{max, Ordering};
use std::fmt::Debug;
use crate::queue::queue::Queue;

type Link<T, U> = Option<Box<BST<T, U>>>;

#[derive(Debug, Clone)]
struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

impl<T, U> BST<T, U>
    where T: Copy + Ord + Debug,
          U: Copy + Debug,
{
    fn new() -> Self {
        Self {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut size: usize) -> usize {
        if self.key.is_none() {
            return size;
        }

        // 将当前节点数加入总节点数（根节点）
        size += 1;

        // 分别计算左、右节点数
        if self.left.is_some() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }

        if self.right.is_some() {
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    // 计算叶子节点个数
    fn leaf_size(&self) -> usize {
        // 都为空，当前节点就是叶节点，返回 1
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        // 计算左、右子树的叶节点数
        let left_leaf_size = match &self.left {
            Some(node) => node.leaf_size(),
            None => 0,
        };

        let right_leaf_size = match &self.right {
            Some(node) => node.leaf_size(),
            None => 0,
        };

        left_leaf_size + right_leaf_size
    }

    // 非叶子节点的个数
    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    // 计算树的深度
    fn depth(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut left_depth = 1;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        max(left_depth, right_depth)
    }

    // 插入节点
    fn insert(&mut self, key: T, val: U) {
        // 没有数据时，直接插入
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                // 存在key，更新val
                Some(k) => {
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }

                    // 未找到相同的 key，需要插入新节点
                    // 先找到需要插入的子树
                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    // 根据节点递归下去，直到插入为止
                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val);
                        },
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        }
                    }
                },
                None => ()
            }
        }
    }

    fn contains(&self, key: &T) -> bool {
        match &self.key {
            None => false,
            Some(k) => {
                match k.cmp(key) {
                    Ordering::Equal => true,
                    Ordering::Less => {
                        match &self.right {
                            Some(node) => node.contains(key),
                            None => false,
                        }
                    }
                    Ordering::Greater => {      // 去左子树找
                        match &self.left {
                            Some(node) => node.contains(key),
                            None => false,
                        }
                    }
                }
            }
        }
    }

    fn min(&self) -> (Option<&T>, Option<&U>) {
        // 最小值一定在左边
        match &self.left {
            Some(node) => node.min(),
            None => match &self.key {
                // Some(key) => (Some(&key), self.val.as_ref()),
                Some(_) => (self.key.as_ref(), self.val.as_ref()),
                None => (None, None),
            }
        }
    }

    fn max(&self) -> (Option<&T>, Option<&U>) {
        // 最大值一定在右边
        match &self.right {
            Some(node) => node.max(),
            None => {
                match &self.key {
                    Some(key) => (self.key.as_ref(), self.val.as_ref()),
                    None => (None, None),
                }
            }
        }
    }

    fn get_left(&self) -> Link<T, U> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T, U> {
        self.right.clone()
    }

    // 获取值引用
    fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => {
                match k.cmp(key) {
                    Ordering::Equal => self.val.as_ref(),
                    Ordering::Less => {
                        match &self.right {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    }
                    Ordering::Greater => {
                        match &self.left {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    }
                }
            }
        }
    }

    fn preorder(&self) {
        println!("key: {:?}, val: {:?}", self.key, self.val);
        match &self.left {
            Some(node) => node.preorder(),
            None => ()
        }
        match &self.right {
            Some(node) => node.preorder(),
            None => ()
        }
    }

    fn inorder(&self) {
        match &self.left {
            Some(node) => node.inorder(),
            None => ()
        }
        println!("key: {:?}, val: {:?}", self.key, self.val);
        match &self.right {
            Some(node) => node.inorder(),
            None => ()
        }
    }

    fn postorder(&self) {
        match &self.left {
            Some(node) => node.postorder(),
            None => ()
        }

        match &self.right {
            Some(node) => node.postorder(),
            None => ()
        }
        println!("key: {:?}, val: {:?}", self.key, self.val);
    }

    fn levelorder(&self) {
        let size = self.size();
        let mut q = Queue::new(size);

        let _r = q.enqueue(Box::new(self.clone()));
        while !q.is_empty() {
            let front = q.dequeue().unwrap();
            println!("key: {:?}, val: {:?}", front.key, front.val);

            match front.get_left() {
                Some(left) => {
                    let _ = q.enqueue(left);
                },
                None => (),
            }
            match front.get_right() {
                Some(right) => {
                    let _ = q.enqueue(right);
                },
                None => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut bst = BST::new();
        bst.insert(8, 'e');
        bst.insert(6, 'c');
        bst.insert(7, 'd');
        bst.insert(5, 'b');
        bst.insert(10, 'g');
        bst.insert(9, 'f');
        bst.insert(11, 'h');
        bst.insert(4, 'a');

        println!("bst is empty: {:?}", bst.is_empty());
        println!("bst size: {}", bst.size());
        println!("bst leaves: {}", bst.leaf_size());
        println!("bst internals: {}", bst.none_leaf_size());
        println!("bst depth: {}", bst.depth());

        let min_kv = bst.min();
        let max_kv = bst.max();
        println!("min key-val: {:?}-{:?}", min_kv.0, min_kv.1);
        println!("max key-val: {:?}-{:?}", max_kv.0, max_kv.1);

        println!("bst contains 5: {}", bst.contains(&5));
        println!("key: 5, val: {:?}", bst.get(&5).unwrap());
    }

    #[test]
    fn order() {
        let mut bst = BST::new();
        bst.insert(8, 'e');
        bst.insert(6, 'c');
        bst.insert(7, 'd');
        bst.insert(5, 'b');
        bst.insert(10, 'g');
        bst.insert(9, 'f');
        bst.insert(11, 'h');
        bst.insert(4, 'a');

        println!("--- internal inorder, preorder, postorder ---");
        bst.inorder();
        println!("-----");
        bst.preorder();
        println!("-----");
        bst.postorder();
        println!("-----");
        bst.levelorder();
        println!("outside inorder, preorder, postorder");
    }
}