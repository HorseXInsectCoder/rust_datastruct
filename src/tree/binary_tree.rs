use std::cmp::{max, Ordering};
use std::fmt::Debug;
use crate::queue::queue::Queue;

type Link<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, Clone, PartialEq)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }

    // 将新的子节点作为根节点的左子节点
    fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    // 计算节点数
    fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut size: usize) -> usize {
        size += 1;

        if !self.left.is_none() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }
        if !self.right.is_none() {
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    fn depth(&self) -> usize {
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

    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key;
    }

    fn min(&self) -> Option<&T> {
        // 最小值一定在最左侧
        match &self.left {
            Some(node) => node.min(),
            None => Some(&self.key)
        }
    }

    fn max(&self) -> Option<&T> {
        // 最大值一定在最右侧
        match &self.right {
            Some(node) => node.max(),
            None => Some(&self.key)
        }
    }

    // 计算叶子节点数
    fn leaf_size(&self) -> usize {
        // 都为空，当前节点就是叶子节点，返回1
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        // 计算左、右子树的叶子节点数
        let left_leaf = match &self.left {
            Some(left) => left.leaf_size(),
            None => 0,
        };
        let right_leaf = match &self.right {
            Some(right) => right.leaf_size(),
            None => 0,
        };

        left_leaf + right_leaf
    }

    // 计算非叶子节点数
    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    fn contains(&self, key: &T) -> bool {
        match self.key.cmp(key) {
            Ordering::Equal => true,
            Ordering::Less => {
                match &self.right {
                    Some(node) => node.contains(key),
                    None => false
                }
            }
            Ordering::Greater => {
                match &self.left {
                    Some(node) => node.contains(key),
                    None => false
                }
            }
        }
    }

    fn preorder(&self) {
        println!("key: {:?}", self.key);
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
        println!("key: {:?}", self.key);
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
        println!("key: {:?}", self.key);
    }

    fn levelorder(&self) {
        let size = self.size();
        let mut q = Queue::new(size);

        let _r = q.enqueue(Box::new(self.clone()));
        while !q.is_empty() {
            // 出队首节点并输出值
            let front = q.dequeue().unwrap();
            println!("key {:?}", front.get_key());

            // 找到子节点并入队
            match front.get_left() {
                Some(node) => {
                    let _r = q.enqueue(node);
                }
                None => {}
            }
            match front.get_right() {
                Some(node) => {
                    let _r = q.enqueue(node);
                }
                None => {}
            }
        }
    }
}

// ------------
// 外部实现
fn preorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        println!("from key: {:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

fn inorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}

fn postorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
    }
}
// ------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut bt = BinaryTree::new(10);
        let root = bt.get_key();
        println!("root key: {:?}", root);

        bt.set_key(11);
        let root = bt.get_key();
        println!("root key: {:?}", root);

        bt.insert_left_tree(2);
        bt.insert_right_tree(18);
        println!("left child: {:?}", bt.get_left());
        println!("right child: {:?}", bt.get_right());

        println!("min key: {:?}", bt.min());
        println!("max key: {:?}", bt.max());

        println!("tree noes： {}", bt.size());
        println!("tree leaves: {}", bt.leaf_size());
        println!("tree internals: {}", bt.none_leaf_size());
        println!("tree depth: {}", bt.depth());
        println!("tree contains '2', {}", bt.contains(&2));
    }

    #[test]
    fn order_test() {
        let mut bt = BinaryTree::new(10);
        bt.insert_left_tree(2);
        bt.insert_right_tree(18);

        println!("internal pre-in-post-level order");
        println!("preorder: ");
        bt.preorder();
        println!("inorder: ");
        bt.inorder();
        println!("postorder: ");
        bt.postorder();
        bt.levelorder();
        println!("outside pre-in-post-level order");
    }
}