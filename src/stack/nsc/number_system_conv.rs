use crate::stack::stack::Stack;

fn divide_by_two(mut dec_num: u32) -> String {
    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap_or_default().to_string();
        bin_str += &rem;
    }
    bin_str
}

fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap_or_default() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_test() {
        let num = 10;
        let bin_str = divide_by_two(num);
        println!("{num} = b{bin_str}");
    }

    #[test]
    fn base_conv_test() {
        let num1 = 10;
        let num2 = 43;
        let bin_str = base_converter(num1, 2);
        let hex_str = base_converter(num2, 16);
        println!("{num1} = b{bin_str}, {num2} = x{hex_str}")
    }
}