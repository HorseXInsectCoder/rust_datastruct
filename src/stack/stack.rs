
#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>
}


impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    // 这里的 IntoIter 是定义结构
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Stack { size: 0, data: vec![] } };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }

}

pub struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

// 可以换成 stack: Stack<&'a T>，初始化的时候麻烦些
pub struct Iter<'a, T: 'a> {
    // stack: Vec<&'a T>,
    stack: Stack<&'a T>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        println!("size: {}, {:?}", s.len(), s);
        println!("pop: {}, size: {:?}", s.pop().unwrap(), s.len());
        println!("empty: {}, {:?}", s.is_empty(), s);

        s.clear();
        println!("{:?}", s);
    }

    #[test]
    fn peek() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        println!("{:?}", s);
        let peek_mut = s.peek_mut();
        if let Some(top) = peek_mut {
            *top = 4;
        }

        println!("top: {:?}", s.peek().unwrap());
        println!("{:?}", s);
    }

    #[test]
    fn iter() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        let sum1 = s.iter().sum::<i32>();
        let mut addend = 0;
        for item in s.iter_mut() {
            *item += 1;
            addend += 1;
        }

        let sum2 = s.iter().sum::<i32>();
        println!("{sum1} + {addend} = {sum2}");
        assert_eq!(9, s.into_iter().sum::<i32>());
    }
}