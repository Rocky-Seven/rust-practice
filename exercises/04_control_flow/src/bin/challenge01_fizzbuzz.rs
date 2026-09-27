// チャレンジ1：FizzBuzz関数
//
// 【課題】
// fizzbuzz(n) : 3と5両方の倍数なら"FizzBuzz"、3の倍数なら"Fizz"、
//               5の倍数なら"Buzz"、それ以外はnを文字列にして返す
//
// ヒント: n.to_string() で数値を文字列に変換できる
//
// 実行  : cargo run --bin challenge01_fizzbuzz
// テスト: cargo test --bin challenge01_fizzbuzz

fn fizzbuzz(n: u32) -> String {
    // TODO: 条件に応じた文字列を返そう
    todo!()
}

fn main() {
    println!("=== チャレンジ1：FizzBuzz関数 ===");

    for n in 1..=30 {
        println!("{}", fizzbuzz(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }
}
