#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<T> {
    cap: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Default> HashMap<T> {
    pub fn new(cap: usize) -> Self {
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);
        for _i in 0..cap {
            slot.push(0);
            data.push(Default::default());
        }
        HashMap {
            cap,
            slot,
            data,
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for &d in self.slot.iter() {
            // 槽中的数据不为0，表示有数据，对len加1
            if d != 0 {
                len += 1;
            }
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for &d in self.slot.iter() {
            if d != 0 {
                empty = false;
                break;
            }
        }
        empty
    }

    pub fn clear(&mut self) {
        let mut slot = Vec::with_capacity(self.cap);
        let mut data = Vec::with_capacity(self.cap);
        for _i in 0..self.cap {
            slot.push(0);
            data.push(Default::default());
        }
        self.slot = slot;
        self.data = data;
    }

    // 得到存放的位置
    pub fn hash(&self, key: usize) -> usize {
        key % self.cap
    }

    pub fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.cap
    }

    pub fn insert(&mut self, key: usize, value: T) {
        if key == 0 {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            // 槽中没有数据，直接插入
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            // 要插入的槽中有数据，寻找下一个可行的插入位置
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                // 没有位置，继续rehash
                next = self.rehash(next);

                // 槽满了就退出（走了一圈，重新回到 pos 仍然找不到位置）
                if next == pos {
                    println!("Error: slot is full!");
                    return;
                }
            }
            // 在找到的槽中插入数据
            // if 0 == self.slot[next] {
            //     self.slot[next] = key;
            //     self.data[next] = value;
            // } else {
            //     self.data[next] = value;
            // }

            self.slot[next] = key;
            self.data[next] = value;
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        // 想要删的槽没有数据
        if 0 == self.slot[pos] {
            None
        } else if key == self.slot[pos] {
            // 找到刚好当前位置就有相同的键，就更新 slot 和 data
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            // 当前位置的找不到相同的键（被rehash到其他地方了）
            let mut data = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if key == self.slot[curr] {
                    // 找到了值，删除数据
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                    found = true;
                } else {
                    // hash 回到最初的位置，说明没有找到
                    if curr == self.rehash(curr) {
                        if curr == pos {
                            stop = true;
                        }
                    }
                }
            }
            data
        }
    }

    pub fn get_pos(&self, key: usize) -> usize {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        // 计算数据的位置
        let pos = self.hash(key);
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        while 0 != self.slot[curr] && !found && !stop {
            if key == self.slot[curr] {
                found = true;
            } else {
                // hash 回到最初的位置，说明没有找到
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        curr
    }

    // 获取 val 的普通引用及可变引用
    pub fn get(&self, key: usize) -> Option<&T> {
        let curr = self.get_pos(key);
        self.data.get(curr)
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        let curr = self.get_pos(key);
        self.data.get_mut(curr)
    }

    pub fn contains(&self, key: usize) -> bool {
        if 0 == key {
            panic!("Error : key_mutt > 0")
        }
        self.slot.contains(&key)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { data: vec![] };
        for item in self.data.iter() {
            iterator.data.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { data: vec![] };
        for item in self.data.iter_mut() {
            iterator.data.push(item);
        }
        iterator
    }
}

struct Iter<'a, T> {
    data: Vec<&'a T>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

struct IterMut<'a, T> {
    data: Vec<&'a mut T>
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_search_basic_test() {
        let mut hmap = HashMap::new(11);
        hmap.insert(2, "dog");
        hmap.insert(3, "tiger");
        hmap.insert(10, "cat");

        println!("empty: {}, size: {:?}", hmap.is_empty(), hmap.len());
        println!("contains key 2: {}", hmap.contains(2));
        println!("key 3: {:?}", hmap.get(3));
        let val_ptr = hmap.get_mut(3).unwrap();
        *val_ptr = "fish";
        println!("key 3 after change: {:?}", hmap.get(3));
        println!("remove key 3: {:?}", hmap.remove(3));
        println!("remove key 3: {:?}", hmap.remove(3));

        hmap.clear();
        println!("empty: {}, size: {}", hmap.is_empty(), hmap.len());
    }

    #[test]
    fn hash_search_iter_test() {
        let mut hmap = HashMap::new(11);
        hmap.insert(2, "dog");
        hmap.insert(3, "tiger");
        hmap.insert(10, "cat");

        for item in hmap.iter() {
            println!("val: {item}")
        }

        for item in hmap.iter_mut() {
            *item = "fish"
        }
        println!();

        for item in hmap.iter() {
            println!("val: {item}")
        }
    }
}