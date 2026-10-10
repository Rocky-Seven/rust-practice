// 練習1：ムーブの基本
//
// 【課題】
// describe(text) : Stringを受け取り、"長さ: {文字数}文字" という
//                   文字列を作って返す関数（所有権は関数に移動する）
//
// ヒント: .chars().count() で文字数が数えられる
//
// 実行  : cargo run --bin ex01_move
// テスト: cargo test --bin ex01_move

fn describe(text: String) -> String {
    // TODO: "長さ: N文字" という文字列を作って返そう
    todo!()
}

fn main() {
    let s = String::from("こんにちはRust");
    println!("{}", describe(s));
    // ここで s をもう一度使おうとするとエラーになる（ムーブ済みのため）
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        assert_eq!(describe(String::from("hello")), "長さ: 5文字");
        assert_eq!(describe(String::from("あいう")), "長さ: 3文字");
    }
}
