use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum = a.iter().sum::<usize>();
    if sum % 10 != 0 {
        println!("No");
        return;
    }
    // 尺取り
    let a = a.into_iter().cycle().take(2 * n).collect::<Vec<_>>();
    let tar = sum / 10;
    let mut size = 0;
    let mut idx = 0;
    for i in 0..2 * n {
        while idx < 2 * n && size < tar {
            size += a[idx];
            idx += 1;
        }
        if size == tar {
            println!("Yes");
            return;
        }
        size -= a[i];
    }
    println!("No");
}
