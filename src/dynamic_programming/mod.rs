pub fn bottom_up_cut_rod(prices : &Vec<i32>, length : usize) -> (Vec<i32>, Vec<i32>) {
    let mut results = vec![0; length+1];
    let mut s = vec![0; length+1];
    results[0] = 0;
    for j in 1..length + 1 {
        let mut max = -1;
        for i in 1..j + 1 {
            if max < prices[i] + results[j - i] {
                max = prices[i] + results[j - i];
                s[j] = i as i32;
            }
        }
        results[j] = max;
    }
    (results, s)
}

pub fn print_cut_rod(prices : &Vec<i32>, length : usize){
    let (results, firsts_cuts) = bottom_up_cut_rod(prices, length);
    for i in 0..prices.len() {
        print!("{}->{} ", i, prices[i])
    }
    println!();
    let (res, f_cut) = bottom_up_cut_rod(prices, length);
    print_cuts(&res, &f_cut, length);
}

fn print_cuts(results : &Vec<i32>, f_cut : &Vec<i32>, length : usize) {
    if length > 0 {
        let rem_len = length - (f_cut[length] as usize);
        print_cuts(results, f_cut, rem_len);
        print!("{} ", f_cut[length]);
    }

}