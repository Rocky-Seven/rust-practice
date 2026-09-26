// 練習2：戻り値
//
// 【課題】
// 1. square(n)    : n の2乗を返す関数
// 2. is_even(n)   : n が偶数なら true、奇数なら false を返す関数
// 3. divide(a, b) : a ÷ b の「商」と「あまり」をタプルで返す関数
//
// ヒント: あまりは % 演算子で求められる（例: 17 % 5 は 2）
//
// 実行  : cargo run --bin ex02_return
// テスト: cargo test --bin ex02_return

fn square(n: i32) -> i32 {
    // TODO: n * n を返そう
    todo!()
}

fn is_even(n: i32) -> bool {
    // TODO: 2で割ったあまりが0かどうかを返そう
    todo!()
}

fn divide(a: i32, b: i32) -> (i32, i32) {
    // TODO: (商, あまり) のタプルを返そう
    todo!()
}

fn main() {
    println!("=== 練習2：戻り値 ===");

    println!("5の2乗: {}", square(5));
    println!("4は偶数？ {}", is_even(4));
    println!("7は偶数？ {}", is_even(7));

    let (quotient, remainder) = divide(17, 5);
    println!("17 ÷ 5 = {} あまり {}", quotient, remainder);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(4), 16);
        assert_eq!(square(-3), 9);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(17, 5), (3, 2));
        assert_eq!(divide(20, 4), (5, 0));
    }
}
