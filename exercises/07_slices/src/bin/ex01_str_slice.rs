// 練習1：文字列スライス
//
// 【課題】
// first_word(text) : &strを受け取り、最初のスペースまでの
//                     単語をスライスとして返す（スペースがなければ全体）
//
// ヒント: ブログ記事内の first_word 関数を参考にしよう
//
// 実行  : cargo run --bin ex01_str_slice
// テスト: cargo test --bin ex01_str_slice

fn first_word(text: &str) -> &str {
    // TODO: 最初の単語を返そう
    todo!()
}

fn main() {
    let sentence = String::from("Rust は 楽しい");
    println!("最初の単語: {}", first_word(&sentence));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }
}