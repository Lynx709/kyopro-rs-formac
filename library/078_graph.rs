use proconio::{fastout, input, marker::Chars};
use std::collections::*;
use std::collections::HashSet;
use std::cmp;
use core::cmp::*;
use std::io;
use num::integer::lcm;
use libm::atan;
use std::convert::TryInto;

type Graph = Vec<Vec<usize>>;

fn main() {
    input!{
        n:usize,
        m:usize,
    }
    let g:Graph = input_graph(n, m);
    let mut ans = 0;
    for i in 0..n{
        if g[i].iter().filter(|&x| x < &i).count() == 1{
            ans += 1;
        }
    }
    println!("{}", ans);

}

fn input_graph(n:usize, m:usize) -> (Graph){
    input!{
        node:[(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    for nodes in node{
        g[nodes.0-1].push(nodes.1-1);
        g[nodes.1-1].push(nodes.0-1);
    }
    return g;
}
