use proconio::{fastout, input};
use std::convert::TryInto;

#[fastout]
fn main(){
    input! {
        n: usize,
        mut a: [i128; n],
    }
 
    let mut ans:i128 = 1;

    if a.contains(&0){
        ans = 0;
    }

    a.sort();

    for i in 0..n {
        ans = ans * a[i];
        if ans > 10u128.pow(18).try_into().unwrap(){
            println!("-1");
            return;
        }
    }
    
    println!("{}", ans);
}