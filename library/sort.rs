use proconio::{fastout, input};


#[fastout]
fn main(){
    input! {
        n:usize,
        mut m:i128,
        mut ab: [(i128, i128); n],
    }
    ab.sort_by(|a, b| a.cmp(&b));
    let mut ans = 0;
    for (a, b) in ab{
        if m < b {
            ans += a * m;
            break;
        } else {
            ans += a * b;
            m -= b;
        }
    }
    println!("{}", ans);
}
