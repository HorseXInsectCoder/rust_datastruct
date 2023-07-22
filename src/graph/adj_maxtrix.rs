use std::fs::File;
use std::{fs, io, usize};
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::path::Path;
use std::str::FromStr;
use crate::graph::error::{MatrixError::*, MatrixError, Result};

#[derive(Debug)]
pub struct AdjMatrix {
    pub v: usize,
    pub e: usize,
    pub adj: Vec<Vec<usize>>,     // 邻接矩阵
}

impl AdjMatrix {
    // 读取文件
    fn new(file_path: &str) -> Self {
        let mut v = 0;
        let mut e = 0;
        if let Ok(content) = AdjMatrix::read_file(file_path) {
            if let Ok((vertex, edge)) = AdjMatrix::read_v_e(&content) {
                v = vertex;
                e = edge;
            }
        };

        Self {
            v,
            e,
            adj: vec![],
        }
    }

    fn init_matrix(&mut self) -> Option<Vec<Vec<usize>>> {

        if let Ok(content) = AdjMatrix::read_file("g.txt") {
            AdjMatrix::read_v_e(&content)
                // .map(|(v, e)| (v, e, self.read_second2end(&content, vec![vec![0; v]; v]).unwrap_or_default()))
                .map(|(v, e)| self.read_second2end(&content, vec![vec![0; v]; v]).unwrap_or_default())
                // print_adj 返回 Result<()>，然后用直接接上 map 处理，map 把调用 self.print_adj 前的数据 (v, e, adj) 返回出去
                // .and_then(|(_, _, adj)| self.print_adj(adj.clone()).map(|_| adj))
                .and_then(|adj| self.print_adj(adj.clone()).map(|_| adj))
                .map(|adj| {
                    println!("print success!");
                    self.adj = adj.clone();
                    adj
                }).ok()
        } else {
            eprintln!("Error reading file");
            None
        }
    }


    // 校验读到的数据，从第二行开始，第一列不能大于顶点数
    fn validate_vertex(&self, v: usize) -> Result<()> {
        // v是第一行读到的总顶点数，num1是从第二行开始读到的顶点编号
        if v < 0 || v >= self.v {
            return Err(InvalidVertexEdge(v, self.v));
        }
        Ok(())
    }

    fn read_second2end(&self, content: &String, mut adj: Vec<Vec<usize>>) -> Result<Vec<Vec<usize>>> {
        let mut num1 = 0;
        let mut num2 = 0;
        // 从第二行开始读取，并赋值给二维数组
        let mut s = content.lines().skip(1);
        while let Some(line) = s.next() {
            let mut iter = line.split_whitespace();
            num1 = usize::from_str(iter.next().unwrap_or_default()).unwrap_or_default();
            num2 = usize::from_str(iter.next().unwrap_or_default()).unwrap_or_default();

            let (v, _) = AdjMatrix::read_v_e(&content).unwrap();

            // 遇到平行边(将要处理的边在前面已经存储过了就是平行边)
            if adj[num1][num2] == 1 {
                return Err(ParallelEdge);
            }

            // 遇到自环边
            if num1 == num2 {
                return Err(SelfLoop);
            }

            if let Ok(_) = self.validate_vertex(num1) {
                adj[num1][num2] = 1;
                adj[num2][num1] = 1;
            }
        }
        Ok(adj)
    }


    fn read_file(file_path: &str) -> Result<String> {
        // let file = File::open(file_path)?;
        // let mut reader = BufReader::new(file);
        // let mut content = String::new();
        // reader.read_to_string(&mut content)?;
        // Ok(content)

        let contents = fs::read_to_string(file_path)
            // .map_err(|e| e.into())
            // .and_then(|s| Ok::<String, io::Error>(s.trim().to_owned()))?;
            .map_err(|_| ReadFileError)
            .and_then(|s| Ok::<String, _>(s.trim().to_owned()))?;

        // .or_else(|e| Err(e));    用 ? 代替 or_else


        Ok(contents)
    }

    // 只读取第一行，(顶点，边数)
    fn read_v_e<S: AsRef<str>>(s: S) -> Result<(usize, usize)> {
        let s = s.as_ref();
        let mut v = 0;
        let mut e = 0;

        if let Some(line) = s.lines().next() {
            let mut iter = line.split_whitespace();
            v = i32::from_str(iter.next().unwrap_or_default()).unwrap_or_default();
            e = i32::from_str(iter.next().unwrap_or_default()).unwrap_or_default();
        }

        if v <= 0 {
            return Err(VertexError);
        }

        Ok((v as usize, e as usize))
    }

    fn print_adj(&self, adj: Vec<Vec<usize>>) -> Result<()> {
        if adj.len() == 0 {
            return Err(PrintMatrixError);
        }

        let mut buffer = io::BufWriter::new(io::stdout());

        for i in 0..adj.len() {
            for j in 0..adj.len() {
                // Error test
                // if i == 1 {
                //     let err = io::Error::new(io::ErrorKind::Other, "oh, error comes");
                //     return Err(PrintMatrixError(err));
                // }
                // 当前的代码通过push_str拼接字符串,可以直接使用write!宏和循环来打印,避免创建String和拼接
                // 因为前面也是io错误，所以如果错误类型是PrintMatrixError(io::Error)，那么可以写成map_err(PrintMatrixError)
                write!(buffer, "{} ", adj[i][j]).map_err(|_| PrintMatrixError)?;
            }
            writeln!(buffer).map_err(|_| PrintMatrixError)?;
        }

        buffer.flush().map_err(|_| PrintMatrixError)?;

        Ok(())
    }

    // 传入两个顶点
    fn has_edge(&self, v: usize, w: usize) -> bool {
        // 对传入的顶点校验
        let _ = self.validate_vertex(v);

        self.adj[v][w] == 1
    }

    // 返回和某个顶点相连的边(只要找到相邻的点，就找到对应的边)，即返回相连的顶点的集合
    fn adj(&self, v: usize) -> Vec<usize> {
        // 校验v是否合法
        let _ = self.validate_vertex(v);

        let mut res = Vec::new();
        for i in 0..self.v {
            if self.adj[v][i] == 1 {
                res.push(i);
            }
        }

        res
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
        let mut adj_matrix = AdjMatrix::new("g.txt");
        let m = adj_matrix.init_matrix();
        println!("--------");
        println!("{:?}", adj_matrix.v);
        println!("{:?}", adj_matrix.e);
        println!("{:?}", m.unwrap());
    }
}