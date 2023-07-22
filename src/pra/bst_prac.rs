use std::cmp::{max, Ordering};
use std::fmt::Debug;
use crate::queue::queue::Queue;

type Link<T, U> = Option<Box<BST<T, U>>>;

#[derive(Clone, Debug)]
struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

impl<T, U> BST<T, U>
    where T: Copy + Debug + Ord,
          U: Copy + Debug
{
    fn new() -> Self {
        Self {
            key: None,
            val: None,
            left: None,
            right: None
        }
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut size: usize) -> usize {
        if self.key.is_none() {
            return size;
        }

        size += 1;

        if self.left.is_some() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }

        // if self.right.is_some() {
        //     size = self.right.as_ref().unwrap().calc_size(size);
        // }
        if let Some(_) = self.right {
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    fn leaf_size(&self) -> usize {
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

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

    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    fn depth(&self) -> usize {
        let mut left_depth = 1;

        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth = right.depth();
        }

        max(left_depth, right_depth)
    }

    fn insert(&mut self, key: T, val: U) {
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }

                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val)
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
                    Ordering::Greater => {
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
        match &self.left {
            Some(node) => {
                node.min()
            },
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None),
            }
        }
    }

    fn max(&self) -> (Option<&T>, Option<&U>) {
        match &self.right {
            Some(node) => node.max(),
            None => match &self.key {
                Some(key) => (Some(&key), self.val.as_ref()),
                None => (None, None)
            }
        }
    }

    fn get_left(&self) -> Link<T, U> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T, U> {
        self.right.clone()
    }

    fn get(&self, key: &T) -> Option<&U> {
        match self.key {
            Some(k) => {
                match k.cmp(key) {
                    Ordering::Equal => self.val.as_ref(),
                    Ordering::Less => {
                        match &self.right {
                            Some(node) => node.get(key),
                            None => None,
                        }
                    }
                    Ordering::Greater => {
                        match &self.left {
                            Some(node) => node.get(key),
                            None => None
                        }
                    }
                }
            },
            None => None
        }
    }

    fn preorder(&self) {
        println!("key: {:?}, val: {:?}", self.key, self.val);
        match &self.left {
            Some(node) => node.preorder(),
            None => (),
        }

        match &self.right {
            Some(node) => node.preorder(),
            None => (),
        }
    }

    fn inorder(&self) {

        match &self.left {
            Some(node) => node.inorder(),
            None => (),
        }
        println!("key: {:?}, val: {:?}", self.key, self.val);

        match &self.right {
            Some(node) => node.inorder(),
            None => (),
        }
    }

    fn postorder(&self) {

        match &self.left {
            Some(node) => node.postorder(),
            None => (),
        }

        match &self.right {
            Some(node) => node.postorder(),
            None => (),
        }
        println!("key: {:?}, val: {:?}", self.key, self.val);
    }

    fn levelorder(&self) {
        let mut size = self.size();
        let mut q = Queue::new(size);

        let _ = q.enqueue(Box::new(self.clone()));

        while !q.is_empty() {
            let front = q.dequeue().unwrap();
            println!("key: {:?}, val: {:?}", front.key, front.val);

            match front.get_left() {
                Some(node) => {
                    let _ = q.enqueue(node);
                },
                None => ()
            }

            match front.get_right() {
                Some(node) => {
                    let _ = q.enqueue(node);
                },
                None => ()
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
        println!("bst depth: {}", bst.depth());
        println!("bst: {:?}", bst);

        bst.insert(8, 'e');
        bst.insert(6, 'c');
        bst.insert(7, 'd');
        bst.insert(5, 'b');
        bst.insert(10, 'g');
        bst.insert(9, 'f');
        bst.insert(11, 'h');
        bst.insert(4, 'a');

        bst.insert(20, 'x');
        bst.insert(18, 'y');
        bst.insert(21, 'z');


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
        bst.insert(5, 'q');
        println!("key: 5, val: {:?}", bst.get(&5).unwrap());

        println!("key: {:?}", bst.key);
    }

    #[test]
    fn order() {
        let mut bst = BST::new();
        // bst.insert(8, 'e');
        // bst.insert(6, 'c');
        // bst.insert(7, 'd');
        // bst.insert(5, 'b');
        // bst.insert(10, 'g');
        // bst.insert(9, 'f');
        // bst.insert(11, 'h');
        // bst.insert(4, 'a');

        bst.insert(2, 'a');
        bst.insert(4, 'b');
        bst.insert(5, 'c');
        bst.insert(7, 'd');
        bst.insert(8, 'e');
        bst.insert(11, 'f');
        bst.insert(17, 'f');
        bst.insert(18, 'h');
        bst.insert(12, 'q');

        // println!("--- internal inorder, preorder, postorder ---");
        // bst.inorder();
        // println!("-----");
        bst.preorder();
        // println!("-----");
        // bst.postorder();
        // println!("-----");
        // bst.levelorder();
        // println!("outside inorder, preorder, postorder");
    }
}