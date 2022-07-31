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
use permutohedron::LexicalPermutation;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;

type Graph = Vec<Vec<usize>>;

fn main() {
    input!{
        n:usize,
        cp:[(usize, usize); n],
        q:usize,
        lr:[(usize, usize); q],
    }
    let mut s:Vec<usize> = Vec::new();
    let mut t:Vec<usize> = Vec::new();
    s.push(0);
    t.push(0);
    for i in 0..n{
        if cp[i].0 == 1{
            s.push((s[i] + cp[i].1));
            t.push(t[i]);
        } else {
            s.push(s[i]);
            t.push((t[i] + cp[i].1));
        }
    }
    for (l, r) in lr{
        println!("{} {}", s[r] - s[l-1], t[r] - t[l-1]);
    }
}
