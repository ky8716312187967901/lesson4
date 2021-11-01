fn get_sum(nums: &[u32]) -> Option<u32> {
    let mut sum_res: u32 = 0;
    for add_num in nums {
        if 4294967295 - add_num < sum_res {
            return None;
        }
        sum_res += add_num;
    }
    Some(sum_res)
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("正常4个 ： sum={:?}", get_sum(&nums));

    let nums = [1, 2, 3, 4, 429];
    println!("正常5个 ： sum={:?}", get_sum(&nums));

    let nums = [1, 33, 4294967295];
    println!("u32溢出异常 ： sum={:?}", get_sum(&nums));
}
