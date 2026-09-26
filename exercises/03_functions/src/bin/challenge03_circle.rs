// チャレンジ3：円の面積と円周
//
// 【課題】
// 1. circle_area(r)          : 半径 r の円の面積を返す（面積 = π × r × r）
// 2. circle_circumference(r) : 半径 r の円の円周を返す（円周 = 2 × π × r）
// 3. circle_info(r)          : (面積, 円周) をタプルで返す
//                              （1と2の関数を呼び出して作ろう）
//
// 実行  : cargo run --bin challenge03_circle
// テスト: cargo test --bin challenge03_circle

const PI: f64 = 3.141592653589793;

fn circle_area(r: f64) -> f64 {
    // TODO: 面積を計算して返そう
    todo!()
}

fn circle_circumference(r: f64) -> f64 {
    // TODO: 円周を計算して返そう
    todo!()
}

fn circle_info(r: f64) -> (f64, f64) {
    // TODO: circle_area と circle_circumference を使って、タプルを返そう
    todo!()
}

fn main() {
    println!("=== チャレンジ3：円の面積と円周 ===");

    let (area, circumference) = circle_info(5.0);
    println!("半径5の円 面積: {:.2}, 円周: {:.2}", area, circumference);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_circle_area() {
        assert!(approx_eq(circle_area(1.0), PI));
        assert!(approx_eq(circle_area(2.0), 4.0 * PI));
    }

    #[test]
    fn test_circle_circumference() {
        assert!(approx_eq(circle_circumference(1.0), 2.0 * PI));
        assert!(approx_eq(circle_circumference(2.0), 4.0 * PI));
    }

    #[test]
    fn test_circle_info() {
        let (area, circumference) = circle_info(2.0);
        assert!(approx_eq(area, 4.0 * PI));
        assert!(approx_eq(circumference, 4.0 * PI));
    }
}
