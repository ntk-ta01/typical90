use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let is_correct = |brackets: &[char]| -> bool {
        let mut stack = vec![];
        for b in brackets.iter() {
            if *b == '(' {
                stack.push(1);
            } else if !stack.is_empty() {
                stack.pop();
            } else {
                return false;
            }
        }
        stack.is_empty()
    };
    let mut ans = vec![];
    for bit in 0..1 << n {
        let mut bs = vec!['('; n];
        for (i, b) in bs.iter_mut().enumerate() {
            if (bit >> i) & 1 == 1 {
                *b = ')';
            }
        }
        if is_correct(&bs) {
            ans.push(bs.into_iter().collect::<String>());
        }
    }
    ans.sort_unstable();
    for a in ans {
        println!("{}", a);
    }
}
