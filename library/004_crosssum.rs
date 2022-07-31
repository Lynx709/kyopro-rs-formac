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
        h:usize,
        w:usize,
        a:[[usize; w]; h],
    }
    let mut ww = vec![0; w];
    let mut hh = vec![0; h];
    for i in 0..w{
        for j in 0..h{
            ww[i] += a[j][i];
        }
    }
    for i in 0..h{
        for j in 0..w{
            hh[i] += a[i][j];
        }
    }
    for i in 0..h{
        for j in 0..w{
            print!("{} ", hh[i] + ww[j] - a[i][j]);
        }
        print!("\n");
    }

}