// 練習2：loop・while・for
//
// 【課題】
// 1. sum_to(n)      : loopを使って、1からnまでの合計を返す関数
// 2. count_digits(n) : whileを使って、正の整数nの桁数を返す関数
// 3. sum_evens(n)    : forを使って、1からnまでの偶数の合計を返す関数
//
// ヒント（count_digits）: nを10で割り続け、0になるまでの回数を数える
//
// 実行  : cargo run --bin ex02_loop
// テスト: cargo test --bin ex02_loop

fn sum_to(n: u32) -> u32 {
    // TODO: loopとbreakを使って、1からnまでの合計を返そう
    todo!()
}

fn count_digits(n: u32) -> u32 {
    // TODO: whileを使って、nの桁数を返そう（例: 123 -> 3）
    todo!()
}

fn sum_evens(n: u32) -> u32 {
    // TODO: forと範囲(1..=n)を使って、偶数だけの合計を返そう
    todo!()
}

fn main() {
    println!("=== 練習2：loop・while・for ===");

    println!("1〜10の合計: {}", sum_to(10));
    println!("12345の桁数: {}", count_digits(12345));
    println!("1〜10の偶数の合計: {}", sum_evens(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_to(1), 1);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(12345), 5);
        assert_eq!(count_digits(7), 1);
        assert_eq!(count_digits(100), 3);
    }

    #[test]
    fn test_sum_evens() {
        assert_eq!(sum_evens(10), 30); // 2+4+6+8+10
        assert_eq!(sum_evens(1), 0);
    }
}
