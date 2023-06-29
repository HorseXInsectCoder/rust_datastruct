use crate::queue::queue::Queue;

fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(num);

    // 把名单全部入队
    for name in names {
        let _nm = q.enqueue(name);
    }

    while q.len() > 1 {
        // 出入栈中的人名，相当于传递山芋
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // 出入栈达到num次，删除一个人名
        let _rm = q.dequeue();

        println!("队列情况：{:?}", q);
    }

    q.dequeue().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_potato_test() {
        let name = vec!["Mon", "Tom", "Kew", "Lisa", "Marry", "Bob"];
        let survivor = hot_potato(name, 8);
        println!("The survival person is {}", survivor);
    }
}