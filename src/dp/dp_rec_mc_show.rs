
// 动态规划找零
// 使用 cashes_used 收集使用过的各种面额的纸币
fn dp_rec_mc_show(cashes: &[u32], amount: u32, min_cashes: &mut [u32], cashes_used: &mut [u32]) -> u32 {
    // 动态收集从 1 到 amount 的所有面额的最小找零纸币数
    // 然后从小到大凑出找零纸币数
    for denm in 1..=amount {
        let mut min_cashes_num = denm;
        let mut used_cashes = 1;            // 最小面额是 1 元

        for c in cashes.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cashes_num = 1 + min_cashes[index];

            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                used_cashes = *c;
            }
        }
        // 更新各种面额对应的最小找零纸币数
        min_cashes[denm as usize] = min_cashes_num;
        cashes_used[denm as usize] = used_cashes;
    }
    min_cashes[amount as usize]
}

fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("¥{curr}");
        amount -= curr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp_rec_mc_show_test() {
        let cashes = [1,5,10,20,50];
        let amount = 51_u32;
        let mut min_cashes = [0; 92];
        let mut cashes_used = [0; 92];
        let cashes_num = dp_rec_mc_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
        println!("Refund for ¥{} need refund {} cashes", amount, cashes_num);
        print_cashes(&cashes_used, amount);
    }
}