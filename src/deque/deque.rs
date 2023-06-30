
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            data: Vec::with_capacity(cap),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    // Vec 的末尾为队首
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string());
        }
        self.data.push(val);

        Ok(())
    }

    // Vec 的首部为队尾
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("No space available".to_string())
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 从队首移除数据
    pub fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 {
            return self.data.pop();
        }
        None
    }

    // 从队尾移除数据
    pub fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 {
            return Some(self.data.remove(0));
        }
        None
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter {
            data: vec![]
        };
        for item in self.data.iter() {
            iterator.data.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut {
            data: vec![]
        };
        for item in self.data.iter_mut() {
            iterator.data.push(item);
        }
        iterator
    }
}

pub struct IntoIter<T>(Deque<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            return Some(self.0.data.remove(0));
        }
        None
    }
}

pub struct Iter<'a, T> {
    data: Vec<&'a T>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() != 0 {
            return Some(self.data.remove(0));
        }
        None
    }
}

pub struct IterMut<'a, T> {
    data: Vec<&'a mut T>
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() != 0 {
            return Some(self.data.remove(0));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deque_basic_test() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);

        if let Err(e) = d.add_front(5) {
            println!("add_front error: {:?}", e);
        }
        assert!(d.add_front(5).is_err());
        println!("{:?}", d);

        match d.remove_rear() {
            Some(val) => println!("remove rear data {val}"),    // 4
            None => println!("empty deque"),
        }

        match d.remove_front() {
            Some(val) => println!("remove rear data {val}"),    // 2
            None => println!("empty deque"),
        }

        println!("empty: {}, len: {}", d.is_empty(), d.len());
        println!("full: {}, {:?}", d.is_full(), d);

        d.clear();
        println!("{:?}", d);
    }

    #[test]
    fn deque_iter_test() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);

        let sum1 = d.iter().sum::<i32>();
        let mut addend = 0;
        for item in d.iter_mut() {
            *item += 1;
            addend += 1;
        }

        let sum2 = d.iter().sum::<i32>();
        println!("{} + {} = {}", sum1, addend, sum2);
        assert_eq!(14, d.into_iter().sum::<i32>());
    }
}