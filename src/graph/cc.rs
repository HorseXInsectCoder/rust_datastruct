use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

// connected component
#[derive(Debug, Clone)]
struct CC {
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    graph: RefCell<Graph>,
    cc: RefCell<usize>,
}

impl CC {
    fn new() -> Self {
        let mut graph = Graph::new("g.txt");

        Graph::init_matrix(&mut graph);

        let v = vec![false; graph.v];

        Self {
            visited: RefCell::new(v),
            order: RefCell::new(vec![]),
            graph: RefCell::new(graph),
            cc: RefCell::new(0)
        }
    }

    fn process(&mut self) {
        for v in 0..self.graph.borrow().v {
            if !self.visited.borrow_mut()[v] {
                self.dfs(v);
                *self.cc.borrow_mut() += 1;
            }
        }
    }

    fn count_cc(&self) -> usize {
        self.cc.take()
    }

    fn order(&self) -> Vec<usize> {
        self.order.borrow().clone()
    }

    fn dfs(&self, v: usize) {

        self.visited.borrow_mut()[v] = true;
        self.order.borrow_mut().push(v);

        let g = self.graph.borrow();
        for w in g.adj(v) {
            if !self.visited.borrow_mut()[*w] {
                self.dfs(*w);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let mut dfs = CC::new();

        dfs.process();

        println!("{:?}", dfs.order());
        println!("{:?}", dfs.count_cc());

    }
}