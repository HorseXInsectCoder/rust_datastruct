use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
struct CycleDetection {
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    graph: RefCell<Graph>,
    has_cycle: RefCell<bool>,
}

impl CycleDetection {
    fn new(file_path: &str) -> Self {
        // let file_path = "g_no_cycle.txt";
        let mut graph = Graph::new(file_path);

        Graph::init_matrix(&mut graph, file_path);

        let v = vec![false; graph.v];

        Self {
            visited: RefCell::new(v),
            order: RefCell::new(vec![]),
            graph: RefCell::new(graph),
            has_cycle: RefCell::new(false),
        }
    }

    fn process(&mut self) {
        for v in 0..self.graph.borrow().v {
            if !self.visited.borrow()[v] {
                if self.dfs(v, v) {                     // 初始化时，parent 是自己
                    *self.has_cycle.borrow_mut() = true;
                    break;                                    // 找到一个环就够了
                }
            }
        }
    }

    fn order(&self) -> Vec<usize> {
        self.order.borrow().clone()
    }

    // 从顶点 v 开始，判断图中是否有环
    fn dfs(&self, v: usize, parent: usize) -> bool {
        self.visited.borrow_mut()[v] = true;
        self.order.borrow_mut().push(v);

        let g = self.graph.borrow();
        for w in g.adj(v) {
            if !self.visited.borrow_mut()[*w] {
                if self.dfs(*w, v) {
                    return true;
                }
            } else if *w != parent {
                // *self.has_cycle.borrow_mut() = true; 优化过后，这句移到上面执行
                return true;
            }
        }
        false
    }

    fn has_cycle(&self) -> bool {
        self.has_cycle.clone().take()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let mut cd = CycleDetection::new("g.txt");
        cd.process();
        println!("{:?}", cd.has_cycle());

        let mut cd2 = CycleDetection::new("g_no_cycle.txt");
        cd2.process();
        println!("{:?}", cd2.has_cycle());
    }
}