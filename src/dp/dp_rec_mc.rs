
// 动态规划找零
// min_cashes[i] 表示金额 i 的最优解
fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从 1 到 amount 的所有面额的最小找零纸币数
    // 然后从小到大凑出找零纸币数
    for denm in 1..=amount {
        // 求最小值时，初始化一个最大值，即如果全部用面额为 1 的纸币的数量
        let mut min_cashes_num = denm;
        for c in cashes.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>() {
            // 减去当前币的币值c（因为当前的币值c就能占1个空间，所以不用考虑当前币值，只需考虑前面的最优解），然后查找前面的最少币数，同时记录总的最少数
            let index = (denm - c) as usize;

            let cashes_num = 1 + min_cashes[index];
            // println!("cashes_num: {}", cashes_num);

            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
            }
        }
        min_cashes[denm as usize] = min_cashes_num;
    }
    // 收集了各种面额的最小找零纸币数，直接返回
    min_cashes[amount as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp_rec_mc_test() {
        let cashes = [1,5,10,20,50];
        let amount = 90_u32;
        let mut min_cashes = [0; 91];
        let cashes_num = dp_rec_mc(&cashes, amount, &mut min_cashes);
        println!("Refund for ¥{} need refund {} cashes", amount, cashes_num);
    }
}