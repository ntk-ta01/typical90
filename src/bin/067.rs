use proconio::input;
fn main() {
    input! {
        mut n: i128,
        k: i128,
    }
    for i in 0..k {
        n = to_10(8, n);
        let mut vec_n = conver_base(n, 9);
        for i in vec_n.iter_mut() {
            if *i == 8 {
                *i = 5;
            }
        }
        n = vec_n
            .iter()
            .map(|i| i.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        if i == k - 1 {
            println!("{}", n);
        }
    }
}

/**
base進数の数を10進数に変換する
*/
fn to_10(base: i128, x: i128) -> i128 {
    let s = x.to_string();
    let mut ret = 0;
    for (i, c) in s.chars().rev().enumerate() {
        ret += (c as i32 - 48) as i128 * num::pow(base, i);
    }
    ret
}

/**
10進数をbase進数に変換する
*/
fn conver_base(x: i128, base: i128) -> Vec<i128> {
    let mut x = x;
    let mut ret = vec![];
    while x > 0 {
        ret.push(x % base);
        x -= ret[ret.len() - 1];
        x /= base;
    }
    if ret.is_empty() {
        ret.push(0);
    }
    ret.reverse();
    ret
}
