use proconio::{fastout, input, marker::Chars};
use std::{collections::*, vec}; use std::collections::HashSet;
use std::cmp; use core::cmp::*; use std::io;
use num::integer::lcm; use libm::atan;
use std::convert::TryInto;
use num::integer::gcd;
use num::integer::sqrt;
use std::boxed::Box;
use std::collections::VecDeque;
use permutohedron::LexicalPermutation;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;
use text_io::read;
use std::collections::BTreeMap;
type Graph = Vec<Vec<usize>>;

fn main() {
    input!{
        n:usize,
    }
    let mut prime_list = vec![];
    for i in 2..=1000000 {
        let mut ok = true;
        for &prime in &prime_list {
            if prime * prime > i {
                break;
            }
            if i % prime == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            prime_list.push(i);
        }
    }

    for i in prime_list{
        println!("{}", i);
    }
}