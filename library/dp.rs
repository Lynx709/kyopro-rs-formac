use proconio::{fastout, input};
use std::cmp::min;
 
#[fastout]
fn main(){
    input! {
        n: usize,
        h: [i128; n],
    }
    let mut dp = vec![100000000000; n+1];
    dp[0] = 0;
 
    for i in 1..n {
        dp[i] = min(dp[i], dp[i - 1] + (h[i] - h[i - 1]).abs());
        if i > 1{
            dp[i] = min(dp[i], dp[i - 2] + (h[i] - h[i - 2]). abs());
        }
    }
 
    println!("{}", dp[n-1]);
}