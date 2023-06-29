use std::collections::VecDeque;

fn main() {
    let mut v = VecDeque::from(vec![1,2,3]);
    v.insert(v.len() - 1, 4);
    println!("{:?}", v);
}
