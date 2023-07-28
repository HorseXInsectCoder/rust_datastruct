use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use crate::graph::graph::Graph;

struct GraphBFS {
    graph: RefCell<Graph>,
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
}

impl GraphBFS {
    fn new(file_path: &str) -> Self {
        let mut graph = Graph::new();
        Graph::init_matrix(&mut graph, file_path);
        let v_size = graph.v;

        Self {
            graph: RefCell::new(graph),
            visited: RefCell::new(vec![false; v_size]),
            order: RefCell::new(vec![])
        }
    }

    fn process(&self) {
        for v in 0..self.graph.borrow().v {
            if !self.visited.borrow()[v] {
                self.bfs(v)
            }
        }
    }

    fn bfs(&self, s: usize) {
        let mut queue = VecDeque::new();

        // 每次入队的第一个元素是传进来的顶点 v
        queue.push_back(s);
        self.visited.borrow_mut()[s] = true;

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
                }
            }
        }
    }

    fn order(&self) -> Vec<usize> {
        self.order.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_test() {
        let bfs = GraphBFS::new("g_bfs.txt");
        bfs.process();

        println!("{:?}", bfs.order());
    }
}