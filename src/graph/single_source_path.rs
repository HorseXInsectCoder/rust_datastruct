use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
struct SingleSourcePath {
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    graph: RefCell<Graph>,

    // 源
    s: usize,

    // 记录源(即记录当前顶点的上一个顶点)
    pre: RefCell<Vec<i32>>,     // 存储每个顶点前面的顶点
}

impl SingleSourcePath {
    // 传入单源的顶点
    fn new(s: usize) -> Self {
        let mut graph = Graph::new("g.txt");

        Graph::init_matrix(&mut graph);

        // 校验传进来的顶点源 s
        let _ = graph.validate_vertex(s);

        Self {
            visited: RefCell::new(vec![false; graph.v]),
            order: RefCell::new(vec![]),
            graph: RefCell::new(graph.clone()),
            s,
            pre: RefCell::new(vec![-1; graph.v]),       // 数组赋初值 -1
        }
    }

    fn process(&mut self) {
        // 只需要针对 s 这个顶点进行 dfs，即当前的联通分量，这也意味着可能不会把整个图都遍历完
        // self.dfs(self.s);
        self.dfs(self.s, self.s)      // 初始调用时，把 parent 传进去，源的 parent 是它自己
    }

    fn order(&self) -> Vec<usize> {
        self.order.borrow().clone()
    }

    // fn dfs(&self, v: usize) {
    //     self.visited.borrow_mut()[v] = true;
    //     self.order.borrow_mut().push(v);
    //
    //     let g = self.graph.borrow();
    //     // 只能知道 v 的下一个顶点，不能知道 v 是从哪来的（即v的上一个顶点是谁）
    //     for w in g.adj(v) {
    //         if !self.visited.borrow_mut()[*w] {
    //             self.dfs(*w);
    //         }
    //     }
    // }

    fn dfs(&self, v: usize, parent: usize) {
        self.visited.borrow_mut()[v] = true;
        self.order.borrow_mut().push(v);

        let g = self.graph.borrow();

        self.pre.borrow_mut()[v] = parent as i32;
        for w in g.adj(v) {
            if !self.visited.borrow()[*w] {
                self.dfs(*w, v);
            }
        }
    }

    // 从源 s 到 t 是否可达
    fn is_connected(&self, t: usize) -> bool {
        let _ = self.graph.borrow().validate_vertex(t);

        // 只需看在深度遍历的过程中，t 有没有被遍历到
        self.visited.borrow()[t]
    }

    // 从源到目标 t 的路径
    fn path(&self, t: usize) -> Vec<usize> {
        let mut res = Vec::new();
        if !self.is_connected(t) {
            return res;
        }

        // 从 t 开始，用 pre 数组往前查
        let mut cur = t;
        while cur != self.s {
            res.push(cur);
            cur = self.pre.borrow()[cur] as usize;
        }
        res.push(self.s);

        // 反转数组，由于是倒查，所以要反转符合用户看的习惯
        res.reverse();
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let mut ss = SingleSourcePath::new(0);

        ss.process();

        // println!("{:?}", ss.order());
        println!("0 -> 6: {:?}", ss.path(6));
        println!("0 -> 5: {:?}", ss.path(5));
        println!("0 -> 4: {:?}", ss.path(4));
        println!("0 -> 3: {:?}", ss.path(3));
        println!("0 -> 2: {:?}", ss.path(2));
        println!("0 -> 1: {:?}", ss.path(1));
    }
}