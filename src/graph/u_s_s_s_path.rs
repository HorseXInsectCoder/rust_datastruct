use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use crate::graph::graph::Graph;

// Unweighted Single Source Shortest Path
struct USSSPath {
    graph: RefCell<Graph>,
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    pre: RefCell<Vec<i32>>,
    source: usize,
    dis: RefCell<Vec<i32>>,
}

impl USSSPath {
    fn new(file_path: &str, source: usize) -> Self {
        let mut graph = Graph::new();
        Graph::init_matrix(&mut graph, file_path);
        let v_size = graph.v;

        Self {
            graph: RefCell::new(graph),
            visited: RefCell::new(vec![false; v_size]),
            order: RefCell::new(vec![]),
            pre: RefCell::new(vec![-1; v_size]),
            source,
            dis: RefCell::new(vec![-1; v_size]),
        }
    }

    fn process(&self) {
        self.bfs(self.source);

        for i in 0..self.graph.borrow().v {
            print!("{} ", self.dis.borrow()[i]);
        }
        println!();
    }

    fn bfs(&self, s: usize) {
        let mut queue = VecDeque::new();

        // 每次入队的第一个元素是传进来的顶点 v
        queue.push_back(s);
        self.visited.borrow_mut()[s] = true;
        self.pre.borrow_mut()[s] = s as i32;
        self.dis.borrow_mut()[s] = 0;

        // 只要队列不空
        while !queue.is_empty() {
            // 首先从队首取出元素
            let v = queue.pop_front().unwrap();

            // 然后把取出来的元素添加到 order 中
            self.order.borrow_mut().push(v);

            // 再对 v 相邻的节点进行遍历
            let g = self.graph.borrow();
            for w in g.adj(v) {
                if !self.visited.borrow()[*w] {
                    queue.push_back(*w);
                    self.visited.borrow_mut()[*w] = true;
                    self.pre.borrow_mut()[*w] = v as i32;

                    // 从源点 s 到 w 的距离是从 s 到 v 再加 1
                    let v_dis = self.dis.borrow()[v];
                    self.dis.borrow_mut()[*w] = v_dis + 1;
                }
            }
        }
    }

    fn order(&self) -> Vec<usize> {
        self.order.borrow().clone()
    }

    fn is_connected_to(&self, t: usize) -> bool {
        let _ = self.graph.borrow().validate_vertex(t);
        self.visited.borrow()[t]
    }

    fn path(&self, target: usize) -> Vec<usize> {
        let mut res = vec![];
        if !self.is_connected_to(target) {
            return res;
        }

        let mut cur = target;
        while cur != self.source {
            res.push(cur);
            cur = self.pre.borrow()[cur] as usize;
        }
        res.push(self.source);

        res.reverse();
        res
    }

    fn dis(&self, target: usize) -> i32 {
        let _ = self.graph.borrow().validate_vertex(target);
        self.dis.borrow()[target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_test() {
        let bfs = USSSPath::new("g_test.txt", 0);
        bfs.process();

        println!("0 -> 6: {:?}", bfs.path(5));
        println!("0 -> 6 dis: {:?}", bfs.dis(5));
    }
}