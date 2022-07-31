use proconio::{fastout, input, marker::Chars};
use std::collections::*;
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

fn main() {
    input! {
        n:usize,
    }
    let vec = divisors(n);
    let veclen = vec.len();
    let mut ans = std::usize::MAX;

    if veclen % 2 == 0{
        for i in 0..veclen/2{
            ans = min(ans, vec[i]+vec[veclen-1-i]-2);
        }
    } else{
        for i in 0..veclen/2{
            ans = min(ans, vec[i]+vec[veclen-1-i]-2);
        }
        ans = min(ans, vec[veclen/2]*2-2);
    }
    println!("{}", ans);

}

fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    // n := i * x とおくと、 i が i > root(n) の時、　i はすでに ある x に探索されているから
    // i <= root(n) まで探索すればよい
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {

        // i で n が割り切れた場合
        if n % i == 0 {
            // 約数リストに格納
            divisors.push(i);

            // n := i * x の x を格納。ただし x := i の時は除く
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort();
    divisors
}