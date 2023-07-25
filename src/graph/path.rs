use std::cell::{Ref, RefCell};
use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
struct Path {
    visited: RefCell<Vec<bool>>,
    order: RefCell<Vec<usize>>,
    graph: RefCell<Graph>,

    // 源
    s: usize,

    // 记录源(即记录当前顶点的上一个顶点)
    // pre: RefCell<Vec<Option<i32>>>,     // 存储每个顶点前面的顶点
    pre: RefCell<Vec<i32>>,     // 存储每个顶点前面的顶点

    // 目标
    t: usize
}

impl Path {
    // 传入单源的顶点
    fn new(s: usize, t: usize) -> Self {
        let mut graph = Graph::new("g.txt");

        Graph::init_matrix(&mut graph);

        // 校验传进来的顶点源 s
        let _ = graph.validate_vertex(s);
        let _ = graph.validate_vertex(t);

        Self {
            visited: RefCell::new(vec![false; graph.v]),
            order: RefCell::new(vec![]),
            graph: RefCell::new(graph.clone()),
            s,
            // pre: RefCell::new(vec![None; graph.v]),       // 数组赋初值 None
            pre: RefCell::new(vec![-1; graph.v]),       // 数组赋初值 None
            t
        }
    }

    fn process(&mut self) {
        // 只需要针对 s 这个顶点进行 dfs，即当前的联通分量，这也意味着可能不会把整个图都遍历完
        // self.dfs(self.s);
        self.dfs(self.s, self.s);      // 初始调用时，把 parent 传进去，源的 parent 是它自己

        // println!();
        // println!("----- visited  -----");
        // for i in self.visited.borrow().iter() {
        //     print!("{} ", i);
        // }
        // println!();
        //
        // println!("----- pre  -----");
        // for i in self.pre.borrow().iter() {
        //     print!("{:?} ", i);
        // }
    }

    fn order(&self) -> Vec<usize> {
        self.order.borrow().clone()
    }

    fn dfs(&self, v: usize, parent: usize) -> bool {
        self.visited.borrow_mut()[v] = true;
        self.order.borrow_mut().push(v);

        // 这句必须在下面的 return 之前，因为要对 pre 先进行赋值。如果访问了，但是没有对 pre 赋值，会默认 None，导致下面出错
        // self.pre.borrow_mut()[v] = Some(parent as i32);
        self.pre.borrow_mut()[v] = parent as i32;

        // 如果已经找到目标顶点，那么就没有必要再去看与目标顶点相连的节点了
        // 例如 1 是目标， 从 0 到 1，到达 1 后，就返回到 0，没有必要再继续从 1 往下遍历
        if v == self.t {
            return true
        }

        let g = self.graph.borrow();


        for w in g.adj(v) {
            if !self.visited.borrow()[*w] {
                // 返回回到 0 后，也没有必要再去遍历其他的节点，因为已经找到了路径
                if self.dfs(*w, v) {
                    return true;
                }
            }
        }

        // 循环结束，还是没有找到 t，结束
        false
    }

    // 从源 s 到 t 是否可达
    fn is_connected(&self) -> bool {
        // 只需看在深度遍历的过程中，t 有没有被遍历到
        self.visited.borrow()[self.t]
    }

    // 从源到目标 t 的路径
    fn path(&self) -> Vec<usize> {
        let mut res = Vec::new();
        if !self.is_connected() {
            return res;
        }

        // 从 t 开始，用 pre 数组往前查
        let mut cur = self.t as i32;
        while cur != self.s as i32 {
            res.push(cur as usize);
            // cur = self.pre.borrow()[cur as usize].unwrap_or_default();
            cur = self.pre.borrow()[cur as usize];
        }
        res.push(self.s);

        res.reverse();
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let mut path = Path::new(0, 6);
        path.process();
        println!("0 -> 6: {:?}", path.path());


        let mut path = Path::new(0, 1);
        path.process();
        println!("0 -> 1: {:?}", path.path());


        let mut path = Path::new(0, 5);
        path.process();
        println!("0 -> 5: {:?}", path.path());
    }
}