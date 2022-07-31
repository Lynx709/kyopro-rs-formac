#[allow(unused_imports)]
use num::{abs};
use proconio::{input,fastout,marker::Chars};

#[fastout]
fn maine() {
    input! {
        a:String,
    }

    let mut counterj:i32 = 0;
    let mut counterf:i32 = 0;

    for i in s {
        match i{
            'a' => { 
                if counterj + counterf < a+b{
                    println!("Yes");
                    counterj += 1;
                } else {
                    println!("No");
                }
            }
            'b' => {
                if counterj + counterf < a+b && counterf < b{
                    println!("Yes");
                    counterf += 1;
                } else {
                    println!("No");
                }
            }
            'c' => {
                println!("No");
            }
            _ => {

            }
        }
    }


}

let num_str = String::from("334");
let num = num_str.parse::<i32>().expect("334 じゃない");
println!("{}", num);
