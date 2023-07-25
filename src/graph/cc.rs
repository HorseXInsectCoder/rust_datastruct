use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

// connected component
#[derive(Debug, Clone)]
struct CC {
    visited: RefCell<Vec<i32>>,
    order: RefCell<Vec<i32>>,
    graph: RefCell<Graph>,
    cc_count: RefCell<i32>,
}


impl CC {
    fn new() -> Self {
        let mut graph = Graph::new("g.txt");

        Graph::init_matrix(&mut graph);

        // 把 visited 数组初始化为 -1
        let v = vec![-1; graph.v];

        Self {
            visited: RefCell::new(v),
            order: RefCell::new(vec![]),
            graph: RefCell::new(graph),
            cc_count: RefCell::new(0)
        }
    }

    fn process(&mut self) {
        for v in 0..self.graph.borrow().v {
            if self.visited.borrow_mut()[v] == -1 {
                self.dfs(v as i32, self.cc_count.clone());
                *self.cc_count.borrow_mut() += 1;
            }
        }
    }

    // 计算联通分量
    fn count_cc(&self) -> i32 {
        // 这里要注意不能直接 take，因为 take 了后，visited 数组会为空，导致 is_connected 方法访问空数组
        for i in self.visited.clone().take() {
            print!("{} ", i);
        }
        println!();
        self.cc_count.clone().take()
    }

    fn order(&self) -> Vec<i32> {
        self.order.borrow().clone()
    }

    // 深度优先遍历
    fn dfs(&self, v: i32, ccid: RefCell<i32>) {

        self.visited.borrow_mut()[v as usize] = ccid.take();
        self.order.borrow_mut().push(v);

        let g = self.graph.borrow();
        for w in g.adj(v as usize) {
            if self.visited.borrow_mut()[*w] == -1 {
                self.dfs(*w as i32, ccid.clone());
            }
        }
    }

    // 判断两个顶点是否在同一个联通分量中
    fn is_connected(&self, v: usize, w: usize) -> bool {
        let _ = self.graph.borrow().validate_vertex(v);
        let _ = self.graph.borrow().validate_vertex(w);
        self.visited.borrow()[v] == self.visited.borrow()[w]
    }

    // 查看整张图有多少联通分量，每个联通分量包含哪些顶点
    fn components(&self) -> Vec<Vec<i32>> {
        // let mut res: Vec<Vec<i32>> = (0..self.cc_count.clone().take()).map(|_| Vec::new()).collect();
        let mut res = vec![Vec::new(); self.cc_count.clone().take() as usize];

        let graph = self.graph.borrow();
        let visited = self.visited.borrow();
        for v in 0..graph.v {
            res[visited[v] as usize].push(v as i32);
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let mut cc = CC::new();

        cc.process();

        println!("{:?}", cc.order());
        println!("{:?}", cc.count_cc());
        // println!("visited: {:?}", cc.visited);

        // visited: 0 0 0 0 0 1 0
        assert_eq!(cc.is_connected(0, 6), true);
        assert_eq!(cc.is_connected(0, 5), false);

        // let comp = cc.components();
        // for ccid in 0..comp.len() {
        //     print!("{}: ", ccid);
        //     for w in &comp[ccid] {
        //         print!("{} ", *w);
        //     }
        //     println!();
        // }
        for (ccid, comp) in cc.components().iter().enumerate() {
            let mut output = format!("{}: ", ccid);
            for w in comp {
                output.push_str(&format!("{} ", w));
            }
            println!("{}", output);
        }
    }
}