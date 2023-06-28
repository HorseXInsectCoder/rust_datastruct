use std::collections::HashMap;
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
    }
}