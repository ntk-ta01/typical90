use proconio::input;
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }
    let mut answer = usize::max_value();
    for i in 0..10000 {
        for j in 0..10000 {
            if n < a * i + b * j {
                continue;
            }
            let less = n - a * i - b * j;
            if less % c != 0 {
                continue;
            }
            let sum = i + j + less / c;
            answer = answer.min(sum);
        }
    }
    println!("{}", answer);
}
