
#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    // 入队：先判断是否有剩余空间，如果有的话，就将数据添加到队列中
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string())
        }

        self.data.insert(0, val);

        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter {
            data: Vec::new()
        };
        for item in self.data.iter() {
            iterator.data.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut {
            data: vec![]
        };
        for item in self.data.iter_mut() {
            iterator.data.push(item);
        }
        iterator
    }
}

struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            // 删除队尾元素，数据整体往左移动
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

struct Iter<'a, T> {
    data: Vec<&'a T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() != 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

struct IterMut<'a, T> {
    data: Vec<&'a mut T>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() != 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_basic_test() {
        let mut q = Queue::new(4);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);
        let _r4 = q.enqueue(4);

        if let Err(e) = q.enqueue(5) {
            println!("Enqueue error: {}", e);
        }
        assert!(q.enqueue(5).is_err());

        if let Some(data) = q.dequeue() {
            println!("dequeue data: {}", data);
        } else {
            println!("empty queue");
        }

        println!("empty: {}, len: {}", q.is_empty(), q.len());
        println!("full: {}", q.is_full());
        println!("q: {:?}", q);
        q.clear();
        println!("{:?}", q);
    }

    #[test]
    fn queue_iter_test() {
        let mut q = Queue::new(4);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);
        let _r4 = q.enqueue(4);
        let sum1 = q.iter().sum::<i32>();
        let mut addend = 0;
        for item in q.iter_mut() {
            *item += 1;
            addend += 1;
        }
        let sum2 = q.iter().sum::<i32>();
        println!("{sum1} + {addend} = {sum2}");
        println!("sum = {}", q.into_iter().sum::<i32>());
        println!("sum2 = {}", sum2);
    }
}