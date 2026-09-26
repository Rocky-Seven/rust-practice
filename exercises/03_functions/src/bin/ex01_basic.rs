// 練習1：関数の基本
//
// 【課題】
// 1. print_greeting(name) : 「こんにちは、{name}さん！」と出力する関数
// 2. add(a, b)            : 2つの整数の和を返す関数
// 3. subtract(a, b)       : a から b を引いた差を返す関数
//
// 実行  : cargo run --bin ex01_basic
// テスト: cargo test --bin ex01_basic

fn print_greeting(name: &str) {
    // TODO: 「こんにちは、太郎さん！」のように出力しよう
    todo!()
}

fn add(a: i32, b: i32) -> i32 {
    // TODO: a と b の和を返そう（最後の行にセミコロンは付けない）
    todo!()
}

fn subtract(a: i32, b: i32) -> i32 {
    // TODO: a から b を引いた差を返そう
    todo!()
}

fn main() {
    println!("=== 練習1：関数の基本 ===");

    print_greeting("太郎");
    println!("3 + 5 = {}", add(3, 5));
    println!("10 - 4 = {}", subtract(10, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(3, 5), 8);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 4), 6);
        assert_eq!(subtract(4, 10), -6);
    }
}
