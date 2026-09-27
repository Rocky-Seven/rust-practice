// チャレンジ3：成績判定関数（match版）
//
// 【課題】
// grade(score) : matchを使って、90以上"A"、70以上"B"、50以上"C"、それ以外"D"を返す
//
// 実行  : cargo run --bin challenge03_grade
// テスト: cargo test --bin challenge03_grade

fn grade(score: u32) -> &'static str {
    // TODO: matchと範囲パターンを使って判定しよう
    todo!()
}

fn main() {
    println!("=== チャレンジ3：成績判定関数 ===");

    let scores = [95, 82, 61, 40];

    for score in scores {
        println!("点数: {}, 評価: {}", score, grade(score));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), "A");
        assert_eq!(grade(82), "B");
        assert_eq!(grade(61), "C");
        assert_eq!(grade(40), "D");
        assert_eq!(grade(90), "A");
        assert_eq!(grade(70), "B");
        assert_eq!(grade(50), "C");
    }
}
