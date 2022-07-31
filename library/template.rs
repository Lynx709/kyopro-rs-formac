use proconio::{fastout, input};
use proconio::marker::Chars;

#[fastout]
fn main(){
    input!{
        h:usize,
        w:usize,
        x:usize,
        y:usize,
        s:[Chars; h],
    }
    let xx = x - 1;
    let yy = y - 1;
    let mut ans = 1;
    let a:char= '.';
    for i in 0.. w-y {
        if s[xx][y+i] == a {
            ans += 1;
        } else {
            break;
        }
    }
    for i in 1.. y{
        if s[xx][yy-i] == a {
            ans += 1;
        } else {
            break;
        }
    }
    for i in 1..=w-x{
        if s[xx+i][yy]  == a{
            ans += 1;
        } else {
            break;
        }
    }
    for i in 0..=(h-x){
        if xx-i == 0{
            break;
        } else if s[xx-i-1][yy] == a {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}