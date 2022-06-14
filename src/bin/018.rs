use proconio::input;
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        query: [f64; q],
    }
    const PI: f64 = std::f64::consts::PI;
    for e in query {
        // e分後の座標
        let y2 = f64::sin(e / t * 2.0 * PI) * -l / 2.0;
        let z2 = f64::cos(e / t * 2.0 * PI) * -l / 2.0 + l / 2.0;
        let g = f64::sqrt(x * x + (y - y2).abs() * (y - y2).abs());
        println!("{:.12}", z2.atan2(g) * 180.0 / PI);
    }
}
