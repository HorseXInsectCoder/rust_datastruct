use std::collections::{HashMap, VecDeque};
use crate::parentheses_matching::parentheses_matching::par_checker3;
use crate::stack::stack::Stack;

fn infix_to_suffix(infix: &str) -> Option<String> {
    // 括号匹配校验
    if !par_checker3(infix) {
        return None;
    }

    // 设置各个运算符的优先级
    let mut op_priority = HashMap::new();
    op_priority.insert("(", 1);
    op_priority.insert(")", 1);
    op_priority.insert("+", 2);
    op_priority.insert("-", 2);
    op_priority.insert("*", 3);
    op_priority.insert("/", 3);

    // ops是一个栈，用于保存运算符
    // suffix用于保存后缀表达式
    let mut ops = Stack::new();
    let mut suffix = Vec::new();

    for token in infix.split_whitespace() {
        // 将数字 0-9 和大写字母 A-Z 入栈
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            suffix.push(token);
        } else if "(" == token {
            // 遇到开始符号，将运算符入栈
            ops.push(token);
        } else if ")" == token {
            // 遇到结束符号，就查看 ops 栈的顶部元素是不是一个"("，如果不是，则将 ops 栈的顶部元素（操作数）放入 suffix 列表
            let mut top = ops.pop().unwrap();
            // 不停地把 ops 栈里的元素弹出，直至遇到 "("，与")"一起丢弃
            while top != "(" {
                suffix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            // 如果遇到运算符，就比较运算符的优先级以决定是否将运算符添加到 suffix 列表中
            // 如果低优先级遇到栈里的高优先级，就直接把栈顶的高优先级运算符弹出的放到 suffix 列表
            while !ops.is_empty() && (op_priority[ops.peek().unwrap()] >= op_priority[token]) {
                suffix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    // 将栈里剩下的操作数放入列表
    while !ops.is_empty() {
        suffix.push(ops.pop().unwrap());
    }

    // 列表中已经是完整的后缀表达式，拼接成字符串
    let mut suffix_str = String::new();
    for c in suffix {
        suffix_str += &c.to_string();
        suffix_str += " ";
    }

    Some(suffix_str)
}

fn suffix_eval(suffix: &str) -> Option<i32> {
    // 后缀表达式至少需要两个操作数和一个运算符，另外还需要两个空格把它们隔开，所以至少是5个字符
    if suffix.len() < 5 {
        return None;
    }

    let mut ops = Stack::new();
    for token in suffix.split_whitespace() {
        if "0" <= token && token <= "9" {
            ops.push(token.parse::<i32>().unwrap());
        } else {
            // 对于减法和除法，有顺序要求
            // 先出栈的是第二个操作数
            let op2 = ops.pop().unwrap();
            let op1 = ops.pop().unwrap();
            let res = do_calc(token, op1, op2);
            ops.push(res);
        }
    }

    Some(ops.pop().unwrap())
}

fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else if "/" == op {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    } else {
        panic!("OperatorError: Invalid operator: {:?}", op);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_test() {
        let infix = "( A + B ) * ( C + D )";
        let suffix = infix_to_suffix(infix);
        match suffix {
            Some(val) => println!("{infix} -> {val}"),
            None => println!("{infix} isn't a correct infix string"),
        }

        let infix2 = "A + B * C + D";
        let suffix2 = infix_to_suffix(infix);
        match suffix2 {
            Some(val) => println!("{infix2} -> {val}"),
            None => println!("{infix2} isn't a correct infix string"),
        }
    }

    #[test]
    fn suffix_eval_test() {
        let suffix = "1 2 + 1 2 + *";
        let res = suffix_eval(suffix);
        match res {
            Some(val) => println!("res = {val}"),
            None => println!("{suffix} isn't a valid suffix"),
        }
    }
}