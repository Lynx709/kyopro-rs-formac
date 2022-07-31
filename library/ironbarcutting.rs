use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;

fn main() {
    input! {
        n:usize,
        a:[isize; n],
    }
    let mut sum:isize = a.iter().sum();
    let mut left = 0;
    let mut ans = std::isize::MAX;
    for i in 0..n{
        let tmp = a[i];
        left += tmp;
        sum -= tmp;
        ans = min(ans, (sum-left).abs());

    }
    println!("{}", ans);
}