use proconio::{fastout, input};

#[fastout]
fn main(){
    input!{
        a:i128,
        v:i128,
        b:i128,
        w:i128,
        t:i128,
    }
    let mut ans = false;
    if a > b{
        if (a-b) % (w-v) == 0 && (a-b) < t*(w-v){
            ans = true;
        }
    } else if b > a{
        if (b-a) % (v-w) == 0 && (b-a) < t*(v-w){
            ans = true;
        }
    }
    if ans == true{
        println!("YES");
    } else {
        println!("NO");
    }
}