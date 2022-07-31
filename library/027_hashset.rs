use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;

fn main() {
    input!{
        n:usize,
        s:[String; n],
    }
    let mut set = HashSet::new();
    let mut ans = 1;
    for i in s{
        if !set.contains(&i) {
            set.insert(i);
            println!("{}",ans);
        }
        ans += 1;
    }

}