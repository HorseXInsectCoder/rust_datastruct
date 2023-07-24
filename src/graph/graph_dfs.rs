use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
struct GraphDFS {
    // visited: Vec<bool>,
    // order: Vec<usize>,
    // graph: Graph,

    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    graph: RefCell<Graph>,
}

impl GraphDFS {
    fn new() -> Self {
        let mut graph = Graph::new("g.txt");

        Graph::init_matrix(&mut graph);

        let v = vec![false; graph.v];

        Self {

            // visited: v,
            // order: vec![],
            // graph,

            visited: RefCell::new(v),
            order: RefCell::new(vec![]),
            graph: RefCell::new(graph),
        }
    }

    fn process(&mut self) {
        // self.dfs(0);

        for v in 0..self.graph.borrow().v {
            if !self.visited.borrow_mut()[v] {
                self.dfs(v);
            }
        }
    }

    fn order(&self) -> Vec<usize> {
        // self.order.clone()
        self.order.borrow().clone()
    }

    fn dfs(&self, v: usize) {
        // self.visited[v] = true;
        // self.order.push(v);

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
        let mut dfs = GraphDFS::new();

        dfs.process();

        println!("{:?}", dfs.order());
    }
}