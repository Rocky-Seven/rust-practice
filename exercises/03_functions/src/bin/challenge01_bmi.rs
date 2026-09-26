// チャレンジ1：BMI計算関数
//
// 【課題】
// calc_bmi(weight_kg, height_cm) : 体重(kg)と身長(cm)から BMI を返す関数
//
// 計算式:
//   身長をメートルに変換（height_cm / 100.0）
//   BMI = 体重 ÷ (身長(m) × 身長(m))
//
// 実行  : cargo run --bin challenge01_bmi
// テスト: cargo test --bin challenge01_bmi

fn calc_bmi(weight_kg: f64, height_cm: f64) -> f64 {
    // TODO: BMI を計算して返そう
    todo!()
}

fn main() {
    println!("=== チャレンジ1：BMI計算関数 ===");

    let bmi = calc_bmi(70.0, 170.0);
    println!("体重70kg、身長170cmのBMI: {:.2}", bmi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_bmi() {
        assert!((calc_bmi(70.0, 170.0) - 24.22).abs() < 0.01);
        assert!((calc_bmi(60.0, 160.0) - 23.44).abs() < 0.01);
    }
}
