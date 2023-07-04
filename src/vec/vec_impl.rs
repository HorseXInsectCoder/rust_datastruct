use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: None
        }
    }
}

#[derive(Debug)]
pub struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    pub fn push(&mut self, val: T) {
        let node = Node::new(val);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();
            // 找到链表的最后一个节点
            for _i in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    // 在尾部添加新的 LVec
    pub fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clear();
    }

    pub fn insert(&mut self, mut index: usize, elem: T) {
        if index >= self.size {
            index = self.size
        }

        let mut node = Node::new(elem);

        if self.is_empty() {
            self.head = Some(Box::new(node));
            // 在链表的头部插入
        } else if index == 0 {
            node.next = self.head.take();
            self.head = Some(Box::new(node));
            // 在链表中间插入
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut node;
        if index == 0 {
            node = self.head.take().unwrap();
            self.head = node.next.take()
            // 对于其他节点，找到待删除节点并处理前后链接
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }
        self.size -= 1;

        Some(node.elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size < 1 {
            return None;
        } else {
            self.remove(self.size - 1)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            item: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            item: self.head.as_deref_mut()
        }
    }

    pub fn print_lvec(&self) {
        if 0 == self.size {
            println!("Empty lvec");
        }
        for item in self.iter() {
            print!("{:?} ", item)
        }
        println!()
    }
}

pub struct IntoIter<T: Copy + Debug>(LVec<T>);
impl<T: Copy + Debug> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = 0;
        if index <= self.0.size {
            let node = self.0.remove(index);
            index += 1;
            return node;
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> {
    item: Option<&'a Node<T>>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.item.map(|node| {
            self.item = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    item: Option<&'a mut Node<T>>
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.item.take().map(|node| {
            self.item = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut lvec1 = LVec::new();
        lvec1.push(10);
        lvec1.push(11);
        lvec1.push(12);
        lvec1.push(13);
        lvec1.insert(0, 9);

        lvec1.print_lvec();

        let mut lvec2 = LVec::new();
        lvec2.insert(0, 8);
        lvec2.append(&mut lvec1);

        println!("len: {:?}", lvec2.len());
        println!("pop: {:?}", lvec2.pop().unwrap());
        println!("remove: {:?}", lvec2.remove(0).unwrap());

        lvec2.print_lvec();
        lvec2.clear();
        lvec2.print_lvec();
    }

    #[test]
    fn iter() {
        let mut lvec = LVec::new();
        lvec.push(10);
        lvec.push(11);
        lvec.push(12);
        lvec.push(13);

        for i in lvec.into_iter() {
            print!("{} ", i);
        }
    }
}