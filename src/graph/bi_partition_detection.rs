use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
struct BiPartitionDetection {
    visited: RefCell<Vec<bool>>,
    graph: RefCell<Graph>,
    colors: RefCell<Vec<i32>>,
    is_bipartite: bool,
}


impl BiPartitionDetection {
    fn new() -> Self {
        // let file_path = "g.txt";
        let file_path = "g_not_bipartite.txt";
        let mut graph = Graph::new();
        Graph::init_matrix(&mut graph, file_path);

        let v = vec![false; graph.v];
        let v_size = v.len();

        Self {
            visited: RefCell::new(v),
            graph: RefCell::new(graph),
            colors: RefCell::new(vec![-1; v_size]),
            is_bipartite: true
        }
    }

    fn process(&mut self) {
        for v in 0..self.graph.borrow().v {
            if !self.visited.borrow()[v] {
                if !self.dfs(v, 0) {     // 起始染色成 0
                    self.is_bipartite = false;
                    break;
                }
            }
        }
    }

    fn dfs(&self, v: usize, color: i32) -> bool {
        self.visited.borrow_mut()[v] = true;
        self.colors.borrow_mut()[v] = color;

        let g = self.graph.borrow();
        for w in g.adj(v) {
            // 之前没有被访问，就要进行染色
            if !self.visited.borrow_mut()[*w] {
                // 取反，如果 v 的颜色是0，那么 w 就是1，反过来，如果 v 是 0，w 就是 1
                // 如果不是二分图就不用继续检测了，直接返回false
                if !self.dfs(*w, 1 - color) {
                    return false;
                }
            } else if self.colors.borrow()[*w] == self.colors.borrow()[v] {
                // 如果 w 顶点已经被访问过，那么应该是已经染上颜色了的。如果这两个相邻顶点的颜色相同，那么这张图就不是二分图
                return false;
            }
        }
        true
    }

    fn is_bipartite(&self) -> bool {
        self.is_bipartite
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bp_test() {
        let mut bp = BiPartitionDetection::new();

        bp.process();

        println!("{:?}", bp.is_bipartite());
    }
}