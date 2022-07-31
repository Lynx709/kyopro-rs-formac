use proconio::{fastout, input};

#[fastout]
fn main(){
    input!{
        s:String,
    }
    for i in b'a'..=b'z'{
        let j = char::from(i);
        if !s.contains(j){
            println!("{}", j);
            return;
        }
    }
    println!("None");
}