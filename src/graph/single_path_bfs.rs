use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use crate::graph::graph::Graph;

struct SinglePathGraphBFS {
    graph: RefCell<Graph>,
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    pre: RefCell<Vec<i32>>,
    source: usize,
}

impl SinglePathGraphBFS {
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
        }
    }

    fn process(&self) {
        self.bfs(self.source)
    }

    fn bfs(&self, source: usize) {
        let mut queue = VecDeque::new();

        // 每次入队的第一个元素是传进来的顶点 v
        queue.push_back(source);
        self.visited.borrow_mut()[source] = true;
        self.pre.borrow_mut()[source] = source as i32;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_test() {
        let bfs = SinglePathGraphBFS::new("g_test.txt", 0);
        bfs.process();

        println!("0 -> 6: {:?}", bfs.path(6));
    }
}