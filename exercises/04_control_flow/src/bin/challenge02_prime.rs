// チャレンジ2：素数判定関数
//
// 【課題】
// is_prime(n) : nが素数かどうかを返す
//   - 2未満は素数ではない
//   - 2からnの平方根まで、割り切れる数がなければ素数
//
// 実行  : cargo run --bin challenge02_prime
// テスト: cargo test --bin challenge02_prime

fn is_prime(n: u32) -> bool {
    // TODO: whileループを使って判定しよう
    todo!()
}

fn main() {
    println!("=== チャレンジ2：素数判定関数 ===");

    print!("2〜50の素数: ");
    for n in 2..=50 {
        if is_prime(n) {
            print!("{} ", n);
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(21), false);
    }
}
