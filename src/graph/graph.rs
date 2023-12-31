use std::collections::BTreeSet;
use std::{fs, io, usize};
use std::io::Write;
use std::str::FromStr;
use crate::graph::error::MatrixError::{InvalidVertexEdge, ParallelEdge, PrintMatrixError, ReadFileError, SelfLoop, VertexError};
use crate::graph::error::Result;


#[derive(Debug, Clone)]
pub struct Graph {
    pub v: usize,
    pub e: usize,
    pub adj: Vec<BTreeSet<usize>>,
}

impl Graph {
    // 读取文件
    pub fn new() -> Self {
        Self {
            v: 0,
            e: 0,
            adj: Default::default()
        }
    }

    pub fn init_matrix(&mut self, file_path: &str)  {
        let _ = Graph::read_file(file_path)
            .map(|content| {
                self.read_data(&content).unwrap_or_default()
            });
    }


    // 校验读到的数据，从第二行开始，第一列不能大于顶点数
    pub fn validate_vertex(&self, v: usize) -> Result<()> {
        // v是第一行读到的总顶点数，num1是从第二行开始读到的顶点编号
        if v < 0 || v >= self.v {
            return Err(InvalidVertexEdge(v, self.v));
        }
        Ok(())
    }


    pub fn read_file(file_path: &str) -> Result<String> {
        let contents = fs::read_to_string(file_path)
            .map_err(|_| ReadFileError)
            .and_then(|s| Ok::<String, _>(s.trim().to_owned()))?;

        Ok(contents)
    }

    // 只读取第一行，(顶点，边数)
    pub fn read_data<S: AsRef<str>>(&mut self, s: S) -> Result<(usize, usize, Vec<BTreeSet<usize>>)> {
        let s = s.as_ref();
        let mut v = 0;
        let mut e = 0;

        if let Some(line) = s.lines().next() {
            let mut iter = line.split_whitespace();
            v = usize::from_str(iter.next().unwrap_or_default()).unwrap_or_default();
            e = usize::from_str(iter.next().unwrap_or_default()).unwrap_or_default();
        }

        if v <= 0 {
            return Err(VertexError);
        }

        self.v = v;
        self.e = e;

        let mut adj: Vec<BTreeSet<usize>> = (0..v).map(|_| BTreeSet::new()).collect();

        // 读取第一行后面的数据
        let mut num1 = 0;
        let mut num2 = 0;
        // 从第二行开始读取，并赋值给二维数组
        let mut s =  s.lines().skip(1);
        while let Some(line) = s.next() {
            let mut iter = line.split_whitespace();
            num1 = usize::from_str(iter.next().unwrap_or_default()).unwrap_or_default();
            num2 = usize::from_str(iter.next().unwrap_or_default()).unwrap_or_default();

            // 遇到平行边(将要处理的边在前面已经存储过了就是平行边)
            if adj[num1].contains(&num2) {
                return Err(ParallelEdge);
            }

            // 遇到自环边
            if num1 == num2 {
                return Err(SelfLoop);
            }

            if let Ok(_) = self.validate_vertex(num1) {
                adj[num1].insert(num2);
                adj[num2].insert(num1);
            }
        }

        self.adj = adj.clone();


        Ok((v as usize, e as usize, adj))
    }

    pub fn print_adj(&self, mut adj: Vec<BTreeSet<usize>>) -> Result<()> {
        if adj.len() == 0 {
            return Err(PrintMatrixError);
        }

        let mut buffer = io::BufWriter::new(io::stdout());

        // 顶点使用v, w这些名称，索引用i, j会更加清晰
        for v in 0..adj.len() {
            writeln!(buffer, "vertex:{v} --> {:?} ", adj[v].iter().map(|w| w)).map_err(|_| PrintMatrixError)?;
        }

        buffer.flush().map_err(|_| PrintMatrixError)?;

        Ok(())
    }

    // 传入两个顶点
    pub fn has_edge(&self, v: usize, w: usize) -> bool {
        // 对传入的顶点校验
        let _ = self.validate_vertex(v);
        let _ = self.validate_vertex(w);

        self.adj[v].contains(&w)
    }

    // 返回和某个顶点相连的边(只要找到相邻的点，就找到对应的边)，即返回相连的顶点的集合
    pub fn adj(&self, v: usize) -> &BTreeSet<usize> {
        // 校验v是否合法
        let _ = self.validate_vertex(v);

        // 直接返回顶点v对应的链表
        return &self.adj[v]
    }

    // 顶点的度(顶点的邻边的个数)
    fn degree(&self, v: usize) -> usize {
        self.adj(v).len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut graph = Graph::new();
        graph.init_matrix("g_test.txt");

        println!("{:?}", graph.v);
        println!("{:?}", graph.e);
        println!("{:?}", graph.adj);
    }
}