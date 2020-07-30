#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut count = 0;
    let mut i = 0;
    loop {
        if i >= n {
            break;
        }
        if p[i] == i + 1 {
            count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    println!("{}", count);
}
