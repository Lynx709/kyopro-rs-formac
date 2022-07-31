// urls:https://qiita.com/aflc/items/f2be832f9612064b12c6

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
        mut s:String,
    }
    let mut frag = true;
    let mut ans = 0;
    while frag {
        if !s.contains("BW"){
            frag = false;
        } else {
            ans += s.match_indices("BW").count();
            s = s.replace("BW", "WB");
        }
    }
    println!("{}", ans);
    
}