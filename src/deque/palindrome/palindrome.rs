use crate::deque::deque::Deque;

pub fn palindrome_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        d.add_rear(c);
    }

    let mut is_pal = true;
    while d.len() > 1 && is_pal {
        // 出队首尾字符
        let head = d.remove_front();
        let tail = d.remove_rear();

        // 比较首尾字符，若不同，则字符串非回文
        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_test() {
        let pal = "rustsur";
        let is_pal = palindrome_checker(pal);
        assert_eq!(is_pal, true);

        let pal = "panda";
        let is_pal = palindrome_checker(pal);
        assert_eq!(is_pal, false);
    }
}