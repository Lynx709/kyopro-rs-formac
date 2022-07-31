use proconio::{fastout, input};
use std::collections::HashMap;
use std::cmp::max;
use std::collections::HashSet;

#[fastout]
fn main() {
    input!{
        n:usize,
        s:String,
    }
    let mut ans = 0;

    for i in 1..n{
        let left = &s[0..i];
        let right = &s[i..n];
        let leftset = left.chars().collect::<HashSet<_>>();
        let rightset = right.chars().collect::<HashSet<_>>();
        let mut counter = 0;
        for c in leftset.iter(){
            if rightset.contains(&c){
                counter+= 1;
            }
        }
        ans = max(ans, counter);
    }
    println!("{}", ans);

}