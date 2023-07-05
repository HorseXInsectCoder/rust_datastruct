
// 递归找零，返回找零纸币的数量。但是加入了 min_cashes 数组记录已经查找过的递归
/// cashes: 币种面额, amount: 想要找零的金额
fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    let mut min_cashes_num = amount;

    if cashes.contains(&amount) {
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        // 找零金额 amount 有最小找零纸币数，直接返回
        return min_cashes[amount as usize];
    } else {
        for c in cashes.iter().filter(|&&c| c <= amount).collect::<Vec<&u32>>() {
            let cashes_num = 1 + rec_mc2(cashes, amount - c, min_cashes);

            // 更新最小找零纸币数
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                min_cashes[amount as usize] = min_cashes_num;
            }
        }
    }
    min_cashes_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rec_mc2_test() {
        let cashes = [1,5,10,20,50];
        let amount = 90_u32;
        let mut min_cashes = [0; 91];
        let cashes_num = rec_mc2(&cashes, amount, &mut min_cashes);
        println!("need refund {} cashes", cashes_num);
    }
}