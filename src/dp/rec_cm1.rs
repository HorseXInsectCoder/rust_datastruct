
// 递归找零，返回找零纸币的数量
/// cashes: 币种面额, amount: 想要找零的金额
fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    // 全用1元纸币时的最少找零纸币数
    let mut min_cashes = amount;

    if cashes.contains(&amount) {
        return 1;
    } else {
        // 提取符合条件的币值（找零的币值肯定要小于找零金额）
        for c in cashes.iter()
            .filter(|&&c| c <= amount)
            .collect::<Vec<&u32>>() {
            // 对 amount减c，这表示使用了一张面额为c的纸币。加 1 表示需要将当前正在使用的一张纸币计算在内
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);

            // num_cashes若比min_cashes小，则更新
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }
    min_cashes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rec_mc1_test() {
        let cashes = [1,5,10,20,50];
        let amount = 31_u32;
        let cashes_num = rec_mc1(&cashes, amount);
        println!("need refund {} cashes", cashes_num);
    }
}