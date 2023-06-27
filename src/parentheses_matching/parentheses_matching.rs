
use crate::stack::stack::Stack;

fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();

    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c{
            stack.push(c);
        } else {
            // 遍历还没结束，栈就已经空了，说明不匹配
            if stack.is_empty() {
                balance = false;
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }

    // 仅当平衡且栈为空时，括号才是匹配的
    balance && stack.is_empty()
}

pub fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";

    opens.find(open) == closers.find(close)
}

pub fn par_checker2(par: &str) -> bool {
    let mut char_list = Vec::new();

    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                // 比较当前括号和栈顶括号是否匹配
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }

            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

pub fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();

    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }
        // 如果是右括号，则判断是否平衡
        if ')' == c || ']' == c || '}' == c{
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parentheses1_test() {
        let sa = "()(())";
        let sb = "()((()";

        let res1 = par_checker1(sa);
        let res2 = par_checker1(sb);


        println!("sa balanced: {res1}, sb balanced:{res2}");
    }

    #[test]
    fn parentheses2_test() {
        let sa = "(){}[]";
        let sb = "(){)[}";
        let sc = "(){}z[]";
        let res1 = par_checker2(sa);
        let res2 = par_checker2(sb);
        let res3 = par_checker2(sb);
        println!("sa balanced: {res1}, sb balanced:{res2}");
        println!("{sa} balanced: {res1}, {sb} balanced: {res2}, {sc} balance: {res3}");
    }

    #[test]
    fn parentheses3_test() {
        let sa = "(2+3){func}[abc]";
        let sb = "(2+3)*(3-1";
        let sc = "(){abc}x[xx]zz";
        let res1 = par_checker3(sa);
        let res2 = par_checker3(sb);
        let res3 = par_checker3(sc);
        println!("{sa} balanced: {res1}, {sb} balanced: {res2}, {sc} balance: {res3}");
    }
}