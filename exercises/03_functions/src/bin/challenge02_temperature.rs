// チャレンジ2：温度変換関数
//
// 【課題】
// 1. c_to_f(c) : 摂氏を華氏に変換する（F = C × 9/5 + 32）
// 2. f_to_c(f) : 華氏を摂氏に変換する（C = (F - 32) × 5/9）
//
// 実行  : cargo run --bin challenge02_temperature
// テスト: cargo test --bin challenge02_temperature

fn c_to_f(c: f64) -> f64 {
    // TODO: 摂氏を華氏に変換して返そう
    todo!()
}

fn f_to_c(f: f64) -> f64 {
    // TODO: 華氏を摂氏に変換して返そう
    todo!()
}

fn main() {
    println!("=== チャレンジ2：温度変換関数 ===");

    let celsius = 25.0;
    let fahrenheit = c_to_f(celsius);

    println!("摂氏{:.1}°C = 華氏{:.1}°F", celsius, fahrenheit);
    println!("華氏{:.1}°F = 摂氏{:.1}°C", fahrenheit, f_to_c(fahrenheit));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_c_to_f() {
        assert!(approx_eq(c_to_f(0.0), 32.0));
        assert!(approx_eq(c_to_f(100.0), 212.0));
        assert!(approx_eq(c_to_f(25.0), 77.0));
    }

    #[test]
    fn test_f_to_c() {
        assert!(approx_eq(f_to_c(32.0), 0.0));
        assert!(approx_eq(f_to_c(212.0), 100.0));
        assert!(approx_eq(f_to_c(77.0), 25.0));
    }
}
