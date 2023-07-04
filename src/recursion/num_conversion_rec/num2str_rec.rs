
const BASE_STR: [&str; 16] = ["0", "1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];

pub fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASE_STR[num as usize].to_string()
    } else {
        // 递归，最前面的数是最深的递归返回值，上层的递归值直接加在后面。最深层的余数 + 上一层的余数 + 再一层的余数 + ... + 第一层
        num2str_rec(num / base, base) + BASE_STR[(num % base) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num2str_rec_test() {
        let num = 100;
        let sb = num2str_rec(num, 2);
        let so = num2str_rec(num, 8);
        let sh = num2str_rec(num, 16);
        println!("{num} = b{sb}, o{so}, x{sh}");

        let num = 1000;
        let sb = num2str_rec(num, 2);
        let so = num2str_rec(num, 8);
        let sh = num2str_rec(num, 16);
        println!("{num} = b{sb}, o{so}, x{sh}");
    }
}