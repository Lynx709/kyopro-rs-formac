use std::boxed::Box;
use std::collections::VecDeque;
use permutohedron::LexicalPermutation;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;

fn main() {
    input!{
        n:usize,
        mut sp:[(String, usize); n],
    }
    let mut vec = vec![];
    for i in 0..n{
        vec.push((sp[i].0.clone(), sp[i].1, i+1));
    }
    vec.sort_by(|a,b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    for (x, y, z) in vec{
        println!("{}", z);
    }
    
}