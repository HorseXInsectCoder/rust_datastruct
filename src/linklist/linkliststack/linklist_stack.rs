
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Link<T>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None
        }
    }
}

#[derive(Debug)]
pub struct LStack<T> {
    size: usize,
    top: Link<T>
}

impl<T> LStack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            top: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.top = None;
        self.size = 0;
    }

    pub fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &(*node).data
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_deref_mut().map(|node| {
            &mut node.data
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.top.as_deref()       // Option<&Node<T>>，如果是 as_ref() 的话，就是 Option<&Box<Node<T>>>
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.top.as_deref_mut()   // Option<&mut Node<T>>
        }
    }
}

pub struct IntoIter<T>(LStack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {            // 这里可以不用 take 的原因是 next 是 &Node<T>，可以有多个不可变引用
            self.next = node.next.as_deref();       // 不能用 as_ref()，因为要返回 T，如果是 as_ref() 的话，这里会返回 Option<&Box<Node<T>>>
            &node.data
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // 不使用 take 的话，提示：cannot move out of `self.next` which is behind a mutable reference
        // next 是一个 &mut Node<T>， 对于一个可变引用，无法同时存在多个指向同一个变量或数据结构的引用
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut s = LStack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        println!("empty: {:?}", s.is_empty());
        println!("top: {:?}, size: {}", s.peek(), s.len());
        println!("pop: {:?}, size: {}", s.pop(), s.len());

        if let Some(data) = s.peek_mut() {
            *data = 4
        }
        println!("top: {:?}, size: {}", s.peek(), s.len());
        println!("{:?}", s);
        s.clear();
        println!("{:?}", s);
    }
}