use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    let mut two = 1;
    let p = prime_factorize(n);
    while two < p {
        two *= 2;
        ans += 1;
    }
    println!("{}", ans);
}

fn prime_factorize(mut n: usize) -> usize {
    let upper = n;
    let mut res = 0;
    for a in (2..).take_while(|&i| i * i <= upper) {
        if n % a != 0 {
            continue;
        }
        let mut ex = 0;
        while n % a == 0 {
            ex += 1;
            n /= a;
        }
        res += ex;
    }
    if n != 1 {
        res += 1;
    }
    res
}
